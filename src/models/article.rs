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
pub struct ArticleEditFields {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feed {
    pub articles: Vec<Article>,
    pub count: u32,
}

#[derive(Debug)]
pub struct FeedOptions {
    pub offset: u32,
    pub limit: u8,
    pub user: Option<String>,
}

impl Default for FeedOptions {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 20,
            user: None,
        }
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct ArticleRow {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: Option<String>,

    pub author: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<ArticleRow> for Article {
    fn from(row: ArticleRow) -> Self {
        Article {
            slug: row.slug,
            title: row.title,
            description: row.description,
            body: row.body,
            created_at: row.created_at,
            updated_at: row.updated_at,

            tags: vec![],       // TODO
            favorited: false,   // TODO
            favorites_count: 0, // TODO
            author: Profile {
                username: row.author,
                bio: row.bio,
                image: row.image,
                following: false, // TODO
            },
        }
    }
}

#[cfg(feature = "ssr")]
macro_rules! feed_query {
    ($query:literal, $options:expr, $($args:tt)*) => ({
        let count: i32 = sqlx::query_scalar(
            concat!("select count(*) from article join user on article.author = user.username ", $query)
        )
        $(.bind($args))*
        .fetch_optional(crate::db::get())
        .await
        .ok().flatten().unwrap_or_default();
        let articles = sqlx::query_as::<_, ArticleRow>(
            concat!("
                select article.*, user.bio, user.image
                from article join user on article.author = user.username ",
                $query,
                " order by article.created_at desc limit ? offset ?")
        )
        $(.bind($args))*
        .bind($options.limit)
        .bind($options.offset)
        .fetch_all(crate::db::get()).await?;
        fill_feed_details(articles, count as u32, $options).await
    })
}

#[cfg(feature = "ssr")]
async fn fill_feed_details(
    articles: Vec<ArticleRow>,
    count: u32,
    options: &FeedOptions,
) -> Result<Feed, sqlx::Error> {
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

    let articles = articles
        .into_iter()
        .map(Article::from)
        .map(move |mut article| {
            let slug = &article.slug;
            if let Some(tags) = tags.get(slug) {
                article.tags.clone_from(tags);
            }
            if let Some(n) = fav_count.get(slug) {
                article.favorites_count = *n as u32;
            }
            article.favorited = favorited.contains(slug);
            article.author.following = following.contains(&article.author.username);
            article
        })
        .collect();

    Ok(Feed { articles, count })
}

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get(slug: &str, for_user: Option<&str>) -> Result<Self, sqlx::Error> {
        let mut article = sqlx::query_as!(
            ArticleRow,
            "
            select article.*, user.bio, user.image
            from article join user on article.author = user.username
            where article.slug = ?
            ",
            slug
        )
        .map(Article::from)
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

    pub async fn for_editing(slug: &str, author: &str) -> Result<ArticleEditFields, sqlx::Error> {
        let article = sqlx::query!(
            "select title, description, body from article where slug = ? and author = ?",
            slug,
            author,
        )
        .fetch_one(crate::db::get())
        .await?;

        let tags = sqlx::query_scalar!("select tag from tag where article = ?", slug)
            .fetch_all(crate::db::get())
            .await?;

        Ok(ArticleEditFields {
            title: article.title,
            description: article.description,
            body: article.body,
            tags,
        })
    }

    fn slug_from_title(title: &str) -> String {
        let mut slug: String = title
            .chars()
            .filter_map(|c| match c {
                ' ' => Some('-'),
                c if c.is_ascii_alphanumeric() => Some(c),
                _ => None,
            })
            .skip_while(|c| *c == '-')
            .collect();
        slug.make_ascii_lowercase();
        if slug.ends_with('-') {
            slug.push('x');
        }
        slug
    }

    fn validate(title: &str, description: &str, body: &str, tags: &[&str]) -> Option<Vec<String>> {
        let mut errors = Vec::new();
        if title.is_empty() {
            errors.push("missing title");
        } else if title.len() > 100 {
            errors.push("too long title");
        }

        if description.is_empty() {
            errors.push("missing description");
        } else if description.len() > 300 {
            errors.push("too long description");
        }

        if body.is_empty() {
            errors.push("missing body");
        } else if body.len() > 20000 {
            errors.push("too long body");
        }

        if tags.iter().any(|tag| {
            tag.len() > 20
                || tag
                    .split('-')
                    .any(|part| part.is_empty() || part.chars().any(|c| !c.is_ascii_lowercase()))
        }) {
            errors.push("invalid tag (must be short, lowercase a-z and in kebab-case)");
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors.into_iter().map(str::to_owned).collect())
        }
    }

    // TODO: the validation errors passed in nested Results is bit weird, but will do for now
    pub async fn create(
        author: &str,
        title: &str,
        description: &str,
        body: &str,
        tags: &[&str],
    ) -> Result<Result<String, Vec<String>>, sqlx::Error> {
        if let Some(errors) = Self::validate(title, description, body, tags) {
            return Ok(Err(errors));
        }

        let slug = Self::slug_from_title(title);

        sqlx::query!(
            "insert into article (slug, title, description, body, author) values (?, ?, ?, ?, ?)",
            slug,
            title,
            description,
            body,
            author
        )
        .execute(crate::db::get())
        .await?;

        Self::add_tags(&slug, tags).await?;

        Ok(Ok(slug))
    }

    pub async fn update(
        author: &str,
        slug: &str,
        title: &str,
        description: &str,
        body: &str,
        tags: &[&str],
    ) -> Result<Option<Vec<String>>, sqlx::Error> {
        if let Some(errors) = Self::validate(title, description, body, tags) {
            return Ok(Some(errors));
        }

        let res = sqlx::query!(
            "
                update article set title = ?, description = ?, body = ?, updated_at = (datetime('now'))
                where slug = ? and author = ?
            ",
            title,
            description,
            body,
            slug,
            author,
        )
        .execute(crate::db::get())
        .await?;

        if res.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }

        Self::clear_tags(slug).await?;
        Self::add_tags(slug, tags).await?;

        Ok(None)
    }

    pub async fn delete(slug: &str) -> Result<(), sqlx::Error> {
        let res = sqlx::query!("delete from article where slug = ?", slug)
            .execute(crate::db::get())
            .await?;
        if res.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    async fn add_tags(slug: &str, tags: &[&str]) -> Result<(), sqlx::Error> {
        for tag in tags {
            sqlx::query!("insert into tag (article, tag) values (?, ?)", slug, tag)
                .execute(crate::db::get())
                .await?;
        }
        Ok(())
    }

    async fn clear_tags(slug: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("delete from tag where article = ?", slug)
            .execute(crate::db::get())
            .await?;
        Ok(())
    }
}

#[cfg(feature = "ssr")]
impl Feed {
    pub async fn feed(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        feed_query!(
            "where article.author in (select followed from follow where follower = ?)",
            options,
            user
        )
    }

    pub async fn global(options: &FeedOptions) -> Result<Self, sqlx::Error> {
        feed_query!("", options,)
    }

    pub async fn by(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        feed_query!("where article.author = ?", options, user)
    }

    pub async fn favorited(user: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        feed_query!(
            "where article.slug in (select article from favorite where user = ?)",
            options,
            user
        )
    }

    pub async fn tag(tag: &str, options: &FeedOptions) -> Result<Self, sqlx::Error> {
        feed_query!(
            "where article.slug in (select article from tag where tag = ?)",
            options,
            tag
        )
    }
}
