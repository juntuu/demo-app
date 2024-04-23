use leptos::*;

use crate::models::user::User;

#[server]
pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    if let Err(e) = User::create(&username, &email, &password).await {
        tracing::error!("error registering user: {:?}", e);
        Err(ServerFnError::ServerError("Could not register".into()))
    } else {
        server::set_username(username).await;
        leptos_axum::redirect("/");
        Ok(())
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    match sqlx::query_scalar!("select password from user where username = ?", username)
        .fetch_optional(crate::db::get())
        .await
    {
        Err(e) => {
            tracing::error!("failed to get user from database for login: {:?}", e);
            Err(ServerFnError::ServerError("Unexpected error".into()))
        }
        Ok(Some(hash)) if crate::auth::password::verify(&password, &hash) => {
            server::set_username(username).await;
            leptos_axum::redirect("/");
            Ok(())
        }
        _ => {
            use_context::<leptos_axum::ResponseOptions>()
                .expect("response options")
                .set_status(http::StatusCode::FORBIDDEN);
            Ok(())
        }
    }
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use_context::<leptos_axum::ResponseOptions>()
        .expect("response options")
        .insert_header(
            http::header::SET_COOKIE,
            http::HeaderValue::from_str("session=; path=/").expect("set cookie header"),
        );
    leptos_axum::redirect("/login");
    Ok(())
}

#[server]
pub async fn logged_in_user() -> Result<User, ServerFnError> {
    if let Some(username) =
        use_context::<http::request::Parts>().and_then(|req| server::get_username(&req.headers))
    {
        User::get(&username).await.map_err(|e| {
            tracing::error!("could not get user: {:?}", e);
            ServerFnError::ServerError("Could not find user".into())
        })
    } else {
        Err(ServerFnError::ServerError("Must be logged in".into()))
    }
}

#[cfg(feature = "ssr")]
pub mod password {
    use argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    };

    /// Create a new password hash for storing into database
    pub fn hash(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("password hashing")
            .to_string()
    }

    /// Check if password matches hashed password
    pub fn verify(password: &str, hash: &str) -> bool {
        match PasswordHash::new(hash) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(e) => {
                tracing::error!("failed to parse password hash for verification: {:?}", e);
                false
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub mod server {
    use super::*;

    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
        middleware::Next,
        response::Response,
    };

    use jsonwebtoken::{decode, DecodingKey, Validation};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenClaims {
        // Username
        pub sub: String,
        pub exp: usize,
    }

    pub async fn auth_middleware(req: Request<Body>, next: Next) -> Response {
        let path = req.uri().path();

        if let Some(username) = get_username(req.headers()) {
            if User::get(&username).await.is_ok() {
                if path.starts_with("/login") || path.starts_with("/register") {
                    return Response::builder()
                        .status(StatusCode::FOUND)
                        .header(header::LOCATION, "/")
                        .body(Body::empty())
                        .expect("response with headers");
                }
                return next.run(req).await;
            } else {
                tracing::info!("user not found");
            }
        }

        // Not authenticated
        if path.starts_with("/settings") || path.starts_with("/editor") {
            // but should be
            Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/login")
                .header(header::SET_COOKIE, "session")
                .body(Body::empty())
                .expect("response with headers")
        } else {
            next.run(req).await
        }
    }

    pub(crate) fn decode_token(
        token: &str,
    ) -> Result<jsonwebtoken::TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
        let secret = std::env!("JWT_SECRET");
        decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
    }

    pub(crate) fn get_username(headers: &http::HeaderMap) -> Option<String> {
        let header = headers.get(http::header::COOKIE)?.to_str().ok()?;
        let token = header
            .split(';')
            .find_map(|x| x.trim_start().strip_prefix("session="))?;
        decode_token(token).ok().map(|jwt| jwt.claims.sub)
    }

    pub async fn set_username(username: String) -> Option<()> {
        let res = use_context::<leptos_axum::ResponseOptions>()?;
        let claims = TokenClaims {
            sub: username,
            exp: (chrono::Utc::now() + chrono::TimeDelta::days(30)).timestamp() as usize,
        };
        let secret = std::env!("JWT_SECRET");
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("encode token");
        res.insert_header(
            http::header::SET_COOKIE,
            http::HeaderValue::from_str(&format!("session={token}; path=/; HttpOnly"))
                .expect("set cookie header"),
        );
        Some(())
    }
}
