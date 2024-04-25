use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn profile(username: &str, for_user: Option<&str>) -> Result<Profile, sqlx::Error> {
        let mut profile = sqlx::query!(
            "select username, bio, image from user where username = ?",
            username,
        )
        .map(|row| Profile {
            username: row.username,
            bio: row.bio,
            image: row.image,
            following: false,
        })
        .fetch_one(crate::db::get())
        .await?;
        if let Some(user) = for_user {
            profile.following = sqlx::query_scalar!(
                "select 1 from follow where follower = ? and followed = ?",
                user,
                username
            )
            .fetch_optional(crate::db::get())
            .await?
            .is_some();
        }
        Ok(profile)
    }

    pub async fn get(username: &str) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "select username, email, bio, image from user where username = ?",
            username
        )
        .fetch_one(crate::db::get())
        .await
    }

    pub async fn create(username: &str, email: &str, password: &str) -> Result<Self, sqlx::Error> {
        let password = crate::auth::password::hash(password);
        sqlx::query!(
            "insert into user (username, email, password) values (?, ?, ?)",
            username,
            email,
            password,
        )
        .execute(crate::db::get())
        .await?;
        Ok(Self {
            username: username.to_owned(),
            email: email.to_owned(),
            bio: None,
            image: None,
        })
    }

    pub async fn update(&self, password: Option<&str>) -> Result<(), sqlx::Error> {
        // TODO: maybe allow changing username
        if let Some(password) = password.map(crate::auth::password::hash) {
            sqlx::query!(
                "update user set
                    email = ?,
                    password = ?,
                    bio = ?,
                    image = ?
                where username = ?
                ",
                self.email,
                password,
                self.bio,
                self.image,
                self.username,
            )
            .execute(crate::db::get())
            .await?;
        } else {
            sqlx::query!(
                "update user set
                    email = ?,
                    bio = ?,
                    image = ?
                where username = ?
                ",
                self.email,
                self.bio,
                self.image,
                self.username,
            )
            .execute(crate::db::get())
            .await?;
        }
        Ok(())
    }
}
