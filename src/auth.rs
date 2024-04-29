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
        let mut err = "Could not register".to_string();
        if let sqlx::Error::Database(db) = e {
            let msg = db.message();
            if let Some(field) = msg.strip_prefix("UNIQUE constraint failed: user.") {
                err = format!("Already taken: {}", field);
            }
        }
        Err(ServerFnError::ServerError(err))
    } else {
        server::set_username(username).await;
        leptos_axum::redirect("/");
        Ok(())
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    if sqlx::query_scalar!("select password from user where username = ?", username)
        .fetch_optional(crate::db::get())
        .await?
        .is_some_and(|hash| crate::auth::password::verify(&password, &hash))
    {
        server::set_username(username).await;
        leptos_axum::redirect("/");
    } else {
        expect_context::<leptos_axum::ResponseOptions>().set_status(http::StatusCode::FORBIDDEN);
    }
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let res = expect_context::<leptos_axum::ResponseOptions>();
    server::clear_session_cookie(&res);
    leptos_axum::redirect("/login");
    Ok(())
}

#[server]
pub async fn logged_in_user() -> Result<Option<User>, ServerFnError> {
    if let Some(username) = authenticated_username() {
        User::get(&username).await.map(Option::Some).map_err(|e| {
            tracing::error!("could not get user: {:?}", e);
            ServerFnError::ServerError("Could not find user".into())
        })
    } else {
        Ok(None)
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
pub fn require_login() -> Result<String, ServerFnError> {
    authenticated_username().ok_or_else(|| ServerFnError::ServerError("Not logged in".into()))
}

#[cfg(feature = "ssr")]
pub fn authenticated_username() -> Option<String> {
    use_context::<http::request::Parts>().and_then(|req| server::get_username(&req.headers))
}

#[cfg(feature = "ssr")]
pub mod server {
    use super::*;

    use axum::{
        body::Body,
        http::{header, HeaderValue, Request, StatusCode},
        middleware::Next,
        response::Response,
    };

    use jsonwebtoken::{decode, DecodingKey, Validation};
    use leptos_axum::ResponseOptions;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenClaims {
        // Username
        pub sub: String,
        pub exp: usize,
    }

    fn set_session_cookie(response_options: &ResponseOptions, token: &str) {
        response_options.insert_header(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!(
                "session={token}; path=/; HttpOnly; SameSite=Strict"
            ))
            .expect("set cookie header"),
        );
    }

    pub(crate) fn clear_session_cookie(response_options: &ResponseOptions) {
        response_options.insert_header(
            header::SET_COOKIE,
            HeaderValue::from_str(
                // See "to remove cookie": https://www.rfc-editor.org/rfc/rfc6265#section-3.1
                "session=; path=/; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
            )
            .expect("set cookie header"),
        );
    }

    fn redirect(path: &str) -> Response {
        Response::builder()
            .status(StatusCode::FOUND)
            .header(header::LOCATION, path)
            .body(Body::empty())
            .expect("redirection response with headers")
    }

    pub async fn auth_middleware(req: Request<Body>, next: Next) -> Response {
        let path = req.uri().path();

        if let Some(username) = get_username(req.headers()) {
            if User::get(&username).await.is_ok() {
                if path.starts_with("/login") || path.starts_with("/register") {
                    return redirect("/");
                }
                return next.run(req).await;
            } else {
                tracing::info!("user not found");
            }
        }

        // Not authenticated
        if path.starts_with("/settings") || path.starts_with("/editor") {
            // but should be
            redirect("/login")
        } else {
            next.run(req).await
        }
    }

    pub(crate) fn get_username(headers: &http::HeaderMap) -> Option<String> {
        let header = headers.get(header::COOKIE)?.to_str().ok()?;
        let token = header
            .split(';')
            .find_map(|x| x.trim_start().strip_prefix("session="))?;
        let secret = std::env!("JWT_SECRET");
        decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .ok()
        .map(|jwt| jwt.claims.sub)
    }

    pub async fn set_username(username: String) -> Option<()> {
        let res = use_context::<ResponseOptions>()?;
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
        set_session_cookie(&res, &token);
        Some(())
    }
}
