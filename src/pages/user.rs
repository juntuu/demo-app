use leptos::*;
use leptos_router::*;

use crate::app::use_current_user;

#[component]
fn ErrorList(#[prop(into)] errors: Signal<Vec<String>>) -> impl IntoView {
    view! {
        <Show when=move || !errors.with(Vec::is_empty)>
            <ul class="error-messages">
                {move || errors().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ul>
        </Show>
    }
}

#[component]
pub fn Login(login: crate::auth::LoginAction) -> impl IntoView {
    let errors = create_rw_signal(Vec::new());
    create_effect(move |_| {
        if let Some(Err(_)) = login.value()() {
            if errors.with_untracked(Vec::is_empty) {
                errors.update(|e| e.push("Incorrect username or password.".into()));
            }
        }
    });

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign in</h1>
                        <p class="text-xs-center">
                            <a href="/register">Need an account?</a>
                        </p>
                        <ErrorList errors=errors/>
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
pub fn Register(register: crate::auth::RegisterAction) -> impl IntoView {
    let errors = create_rw_signal(Vec::new());
    create_effect(move |_| {
        if let Some(Err(err)) = register.value()() {
            let msg = if let ServerFnError::ServerError(msg) = err {
                msg
            } else {
                "Something went wrong".to_string()
            };
            if errors.with_untracked(|e| !e.contains(&msg)) {
                errors.update(|e| e.push(msg));
            }
        }
    });

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign up</h1>
                        <p class="text-xs-center">
                            <a href="/login">Have an account?</a>
                        </p>
                        <ErrorList errors=errors/>
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
    use super::profile::profile_link;

    let username = crate::auth::require_login()?;
    let link = profile_link(&username);

    crate::models::user::User {
        username,
        email,
        bio,
        image,
    }
    .update(password.as_deref())
    .await?;
    leptos_axum::redirect(&link);
    Ok(())
}

// TODO: propagate changes to other part of app e.g. profile image
#[component]
pub fn Settings(logout: crate::auth::LogoutAction) -> impl IntoView {
    let settings = create_server_action::<Settings>();

    // Hack to signal about update
    {
        let update = logout.version();
        let result = settings.value();
        create_effect(move |_| {
            if let Some(Ok(_)) = result() {
                // Only notify after successful action
                update.update(|n| *n += 1);
            }
        });
    }

    let user = use_current_user();
    let settings_form = move || {
        user().map(|user| {
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
                            >
                                {user.bio.unwrap_or_default()}
                            </textarea>
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
    };

    view! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Your Settings</h1>
                        <Suspense>{settings_form}</Suspense>
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
