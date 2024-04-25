use serde::{Deserialize, Serialize};

use super::user::Profile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: Option<String>,

    // Indirect fields
    pub tags: Vec<String>,
    pub favorited: bool,
    pub favorites_count: u32,
    pub author: Profile,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feed {
    pub articles: Vec<Article>,
}

pub struct FeedOptions {
    pub after: String,
    pub limit: u8,
    pub user: Option<String>,
}

impl Default for FeedOptions {
    fn default() -> Self {
        Self {
            after: "~".into(), // greater than any date string
            limit: 20,
            user: None,
        }
    }
}

#[cfg(feature = "ssr")]
macro_rules! feed_query {
    ($query:expr, $($args:tt)*) => ({
        sqlx::query!($query, $($args)*)
            .map(|row| Article {
                slug: row.slug,
                title: row.title,
                description: row.description,
                body: row.body,
                created_at: row.created_at,
                updated_at: row.updated_at,

                tags: vec![], // TODO
                favorited: false, // TODO
                favorites_count: 0, // TODO
                author: Profile {
                    username: row.author,
                    bio: row.bio,
                    image: row.image,
                    following: false, // TODO
                },
            })
    })
}

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get(slug: &str, for_user: Option<&str>) -> Result<Self, sqlx::Error> {
        let mut article = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.slug = ?
            ",
            slug,
        )
        .fetch_one(crate::db::get())
        .await?;

        // FIXME: sqlx does not support subqueries (at least properly).
        // Thus we need to fill in some details here with extra queries.
        article.tags = sqlx::query_scalar!("select tag from tag where article = ?", slug)
            .fetch_all(crate::db::get())
            .await?;

        article.favorites_count =
            sqlx::query_scalar!("select count(*) from favorite where article = ?", slug)
                .fetch_optional(crate::db::get())
                .await?
                .unwrap_or_default() as u32;

        if let Some(user) = for_user {
            article.favorited = sqlx::query_scalar!(
                "select article from favorite where article = ? and user = ?",
                slug,
                user
            )
            .fetch_optional(crate::db::get())
            .await?
            .is_some();
            let author = &article.author.username;
            article.author.following = sqlx::query_scalar!(
                "select followed from follow where followed = ? and follower = ?",
                author,
                user
            )
            .fetch_optional(crate::db::get())
            .await?
            .is_some();
        }

        Ok(article)
    }
}

#[cfg(feature = "ssr")]
impl Feed {
    async fn fill_details(
        articles: Vec<Article>,
        options: &FeedOptions,
    ) -> Result<Self, sqlx::Error> {
        // FIXME: sqlx does not support subqueries (at least properly).
        // Thus we need to fill in some details here with extra queries.
        use std::collections::{HashMap, HashSet};

        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        sqlx::query!("select tag, article from tag")
            .fetch_all(crate::db::get())
            .await?
            .into_iter()
            .for_each(|row| tags.entry(row.article).or_default().push(row.tag));

        let fav_count: HashMap<_, _> =
            sqlx::query!("select article, count(*) as count from favorite group by article")
                .fetch_all(crate::db::get())
                .await?
                .into_iter()
                .map(|row| (row.article, row.count))
                .collect();

        let favorited: HashSet<String> = if let Some(user) = &options.user {
            sqlx::query_scalar!("select article from favorite where user = ?", user)
                .fetch_all(crate::db::get())
                .await?
                .into_iter()
                .collect()
        } else {
            HashSet::new()
        };

        let following: HashSet<String> = if let Some(user) = &options.user {
            sqlx::query_scalar!("select followed from follow where follower = ?", user)
                .fetch_all(crate::db::get())
                .await?
                .into_iter()
                .collect()
        } else {
            HashSet::new()
        };

        let mut articles = articles;
        for article in &mut articles {
            let slug = &article.slug;
            if let Some(tags) = tags.get(slug) {
                article.tags.clone_from(tags);
            }
            if let Some(n) = fav_count.get(slug) {
                article.favorites_count = *n as u32;
            }
            article.favorited = favorited.contains(slug);
            article.author.following = following.contains(&article.author.username);
        }

        Ok(Self { articles })
    }

    pub async fn feed(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        let articles = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.created_at < ?1 and article.author in (
                select followed from follow where follower = ?3
            )
            order by article.created_at desc
            limit ?2
            ",
            options.after,
            options.limit,
            user,
        )
        .fetch_all(crate::db::get())
        .await?;
        Self::fill_details(articles, options).await
    }

    pub async fn global(options: &FeedOptions) -> Result<Self, sqlx::Error> {
        let articles = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.created_at < ?
            order by article.created_at desc
            limit ?
            ",
            options.after,
            options.limit,
        )
        .fetch_all(crate::db::get())
        .await?;
        Self::fill_details(articles, options).await
    }

    pub async fn by(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        let articles = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.created_at < ?1 and article.author = ?3
            order by article.created_at desc
            limit ?2
            ",
            options.after,
            options.limit,
            user,
        )
        .fetch_all(crate::db::get())
        .await?;
        Self::fill_details(articles, options).await
    }

    pub async fn favorited(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        let articles = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.created_at < ?1 and article.slug in (
                select article from favorite where user = ?3
            )
            order by article.created_at desc
            limit ?2
            ",
            options.after,
            options.limit,
            user,
        )
        .fetch_all(crate::db::get())
        .await?;
        Self::fill_details(articles, options).await
    }

    pub async fn tag(tag: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        let articles = feed_query!(
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.created_at < ?1 and article.slug in (
                select article from tag where tag = ?3
            )
            order by article.created_at desc
            limit ?2
            ",
            options.after,
            options.limit,
            tag,
        )
        .fetch_all(crate::db::get())
        .await?;
        Self::fill_details(articles, options).await
    }
}
