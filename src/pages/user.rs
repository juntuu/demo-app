use leptos::*;
use leptos_router::*;

use crate::{error_template::error_boundary_fallback, models::user::User};

use super::profile::profile_link;

type VoidAction<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn Login(login: VoidAction<crate::auth::Login>) -> impl IntoView {
    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign in</h1>
                        <p class="text-xs-center">
                            <a href="/register">Need an account?</a>
                        </p>

                        <Show when=move || {
                            login.value().with(|val| val.as_ref().is_some_and(|x| x.is_err()))
                        }>
                            <ul class="error-messages">
                                <li>Incorrect username or password.</li>
                            </ul>
                        </Show>

                        <ActionForm action=login>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="username"
                                    placeholder="Username"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    name="password"
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right">
                                Sign in
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Register(register: VoidAction<crate::auth::Register>) -> impl IntoView {
    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign up</h1>
                        <p class="text-xs-center">
                            <a href="/login">Have an account?</a>
                        </p>

                        <ul class="error-messages">
                            <li>That email is already taken</li>
                        </ul>

                        <ActionForm action=register>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="username"
                                    placeholder="Username"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    name="email"
                                    placeholder="Email"
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    name="password"
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button type="submit" class="btn btn-lg btn-primary pull-xs-right">
                                Sign up
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[server]
async fn settings(
    email: String,
    image: Option<String>,
    bio: Option<String>,
    password: Option<String>,
) -> Result<(), ServerFnError> {
    let username = crate::auth::require_login()?;
    let link = profile_link(&username);

    User {
        username,
        email,
        bio,
        image,
    }
    .update(password.as_deref())
    .await
    .map(|_| leptos_axum::redirect(&link))
    .map_err(|e| {
        tracing::error!("failed to update user settings: {:?}", e);
        ServerFnError::ServerError("Could not update settings".into())
    })
}

#[server]
async fn get_user() -> Result<User, ServerFnError> {
    let user = crate::auth::require_login()?;
    crate::models::user::User::get(&user).await.map_err(|e| {
        tracing::error!("failed to get user: {:?}", e);
        ServerFnError::ServerError("Could not get user".into())
    })
}

// TODO: propagate changes to other part of app e.g. profile image
#[component]
pub fn Settings(logout: VoidAction<crate::auth::Logout>) -> impl IntoView {
    let settings = create_server_action::<Settings>();
    let user = create_blocking_resource(|| (), |_| get_user());
    view! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Your Settings</h1>
                        <Suspense>
                            <ErrorBoundary fallback=error_boundary_fallback>
                                {move || {
                                    user()
                                        .map(|u| {
                                            u.map(|user| {
                                                view! {
                                                    <ActionForm action=settings>
                                                        <fieldset>
                                                            <fieldset class="form-group">
                                                                <input
                                                                    class="form-control"
                                                                    type="text"
                                                                    placeholder="URL of profile picture"
                                                                    name="image"
                                                                    value=user.image
                                                                />
                                                            </fieldset>
                                                            <fieldset class="form-group">
                                                                <textarea
                                                                    class="form-control form-control-lg"
                                                                    rows="8"
                                                                    placeholder="Short bio about you"
                                                                    name="bio"
                                                                    value=user.bio
                                                                ></textarea>
                                                            </fieldset>
                                                            <fieldset class="form-group">
                                                                <input
                                                                    class="form-control form-control-lg"
                                                                    type="text"
                                                                    placeholder="Email"
                                                                    name="email"
                                                                    value=user.email
                                                                />
                                                            </fieldset>
                                                            <fieldset class="form-group">
                                                                <input
                                                                    class="form-control form-control-lg"
                                                                    type="password"
                                                                    placeholder="New Password"
                                                                    name="password"
                                                                />
                                                            </fieldset>
                                                            <button
                                                                type="submit"
                                                                disabled=settings.pending()
                                                                class="btn btn-lg btn-primary pull-xs-right"
                                                            >
                                                                Update Settings
                                                            </button>
                                                        </fieldset>
                                                    </ActionForm>
                                                }
                                            })
                                        })
                                }}

                            </ErrorBoundary>
                        </Suspense>
                        <hr/>
                        <ActionForm action=logout>
                            <button type="submit" class="btn btn-outline-danger">
                                Or click here to logout.
                            </button>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}
