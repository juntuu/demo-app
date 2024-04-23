use serde::{Deserialize, Serialize};

use super::user::Profile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    id: i64,
    body: String,
    created_at: String,
    author: Profile,
}

#[cfg(feature = "ssr")]
impl Comment {
    pub async fn for_article(slug: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query!(
            "
            select comment.*, user.image
            from comment
            join user on comment.user = user.username
            where comment.article = ?
            order by comment.created_at
            ",
            slug
        )
        .map(|row| Self {
            id: row.id,
            body: row.body,
            created_at: row.created_at,
            author: Profile {
                username: row.user,
                image: row.image,
                // TODO: fetch if needed on the frontend
                bio: None,
                following: false,
            },
        })
        .fetch_all(crate::db::get())
        .await
    }

    pub async fn create(slug: &str, user: &str, body: &str) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            "
            insert into comment (article, user, body)
            values (?, ?, ?)
            ",
            slug,
            user,
            body
        )
        .execute(crate::db::get())
        .await?;

        Ok(res.last_insert_rowid())
    }

    pub async fn delete(id: i64, user: &str) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            "
            delete from comment
            where id = ? and user = ?
            ",
            id,
            user,
        )
        .execute(crate::db::get())
        .await?;
        if res.rows_affected() == 1 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
