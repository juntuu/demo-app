#![allow(clippy::empty_docs)]

use crate::{
    error_template::{error_boundary_fallback, AppError, ErrorTemplate},
    models::{
        article::{Article, Feed},
        user::{Profile, User},
    },
    pages::{article, editor},
};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};

pub(crate) fn use_current_user() -> Signal<Option<User>> {
    expect_context()
}

type VoidAction<T> = Action<T, Result<(), ServerFnError>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let login = create_server_action::<crate::auth::Login>();
    let logout = create_server_action::<crate::auth::Logout>();
    let register = create_server_action::<crate::auth::Register>();

    let versions = (login.version(), logout.version(), register.version());
    let user = create_blocking_resource(
        move || (versions.0(), versions.1(), versions.2()),
        |_| crate::auth::logged_in_user(),
    );
    let maybe_user = Signal::derive(move || user().and_then(Result::ok).flatten());
    provide_context(maybe_user);

    view! {
        <Title text="Conduit"/>

        // Import Ionicon icons & Google Fonts our Bootstrap theme relies on
        <Stylesheet href="//code.ionicframework.com/ionicons/2.0.1/css/ionicons.min.css"/>
        <Stylesheet href="//fonts.googleapis.com/css?family=Titillium+Web:700|Source+Serif+Pro:400,700|Merriweather+Sans:400,700|Source+Sans+Pro:400,300,600,700,300italic,400italic,600italic,700italic"/>
        // Import the custom Bootstrap 4 theme from our hosted CDN
        <Stylesheet href="//demo.productionready.io/main.css"/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/demo-app.css"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }
        }>

            <Routes>
                <Route
                    path="/"
                    // To force fetching current user on server
                    ssr=SsrMode::PartiallyBlocked
                    view=move || {
                        view! {
                            <header>
                                <Nav/>
                            </header>
                            <main>
                                <Outlet/>
                            </main>
                            <footer>
                                <div class="container">
                                    <a href="/" class="logo-font">
                                        conduit
                                    </a>
                                    <span class="attribution">
                                        "An interactive learning project from "
                                        <a href="https://thinkster.io">Thinkster</a>
                                        ". Code & design licensed under MIT."
                                    </span>
                                </div>
                            </footer>
                        }
                    }
                >

                    <Route path="" view=HomePage>
                        <Route
                            path=""
                            view=move || {
                                view! {
                                    <Feed kind=FeedKind::Global>
                                        <Show when=move || maybe_user.with(Option::is_some)>
                                            <NavLink href="/feed">Your Feed</NavLink>
                                        </Show>
                                        <NavLink href="">Global Feed</NavLink>
                                    </Feed>
                                }
                            }
                        />

                        <Route
                            path="/feed"
                            view=move || {
                                view! {
                                    <Feed kind=FeedKind::Feed>
                                        <Show when=move || maybe_user.with(Option::is_some)>
                                            <NavLink href="">Your Feed</NavLink>
                                        </Show>
                                        <NavLink href="/">Global Feed</NavLink>
                                    </Feed>
                                }
                            }
                        />

                        <Route
                            path="/tag/:tag"
                            view=move || {
                                let params = use_params_map();
                                let tag = move || {
                                    params.with(|map| map.get("tag").cloned().unwrap_or_default())
                                };
                                view! {
                                    <Feed kind=Signal::derive(move || FeedKind::Tag(tag()))>
                                        <Show when=move || maybe_user.with(Option::is_some)>
                                            <NavLink href="/feed">Your Feed</NavLink>
                                        </Show>
                                        <NavLink href="/">Global Feed</NavLink>
                                        <NavLink href=""># {tag}</NavLink>
                                    </Feed>
                                }
                            }
                        />

                    </Route>
                    <Route path="/login" view=move || view! { <Login login=login/> }/>
                    <Route path="/register" view=move || view! { <Register register=register/> }/>
                    <Route path="/settings" view=move || view! { <Settings logout=logout/> }/>
                    <Route path="/profile/:username" view=Profile>
                        // TODO: this also fails with TrailingSlash::Redirect, so giving up on that
                        // no the routing is bit more fiddly, but whatever
                        // TODO: maybe add redirection logic on 404 to strip trailing /
                        <Route path="/" view=|| view! { <ProfileFeed/> }/>
                        <Route path="/favorites" view=|| view! { <ProfileFeed favorites=true/> }/>
                    </Route>
                    <Route path="/article/:slug" view=article::Article/>
                    <Route path="/editor" view=editor::New/>
                    <Route path="/editor/:slug" view=editor::Edit/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn NavLink(#[prop(into)] href: MaybeSignal<String>, children: Children) -> impl IntoView {
    view! {
        <li class="nav-item">
            <A class="nav-link" active_class="active" href=href exact=true>
                {children()}
            </A>
        </li>
    }
}

pub(crate) const NBSP: &str = "\u{A0}";

#[component]
fn Nav() -> impl IntoView {
    let user = use_current_user();
    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <A class="navbar-brand" href="/">
                    conduit
                </A>
                <ul class="nav navbar-nav pull-xs-right">
                    <NavLink href="/">Home</NavLink>
                    <Suspense>
                        <Show
                            when=move || user.with(Option::is_none)
                            fallback=move || {
                                user()
                                    .map(|user| {
                                        view! {
                                            <NavLink href="/editor">
                                                <i class="ion-compose"></i>
                                                {NBSP}
                                                New Article
                                            </NavLink>
                                            <NavLink href="/settings">
                                                <i class="ion-gear-a"></i>
                                                {NBSP}
                                                Settings
                                            </NavLink>
                                            <NavLink href=profile_link(&user.username)>
                                                <ProfileImg src=user.image class="user-pic"/>
                                                {user.username}
                                            </NavLink>
                                        }
                                    })
                            }
                        >

                            // Either logged out, or fetching current user info
                            <NavLink href="/login">Sign in</NavLink>
                            <NavLink href="/register">Sign up</NavLink>
                        </Show>
                    </Suspense>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn TagLink(
    #[prop(into)] tag: MaybeSignal<String>,
    #[prop(optional)] outline: bool,
) -> impl IntoView {
    let href = {
        let tag = tag.clone();
        move || format!("/tag/{}", tag())
    };
    let class = if outline {
        "tag-pill tag-default tag-outline"
    } else {
        "tag-pill tag-default"
    };
    view! {
        <li>
            <A href=href class=class>
                {tag}
            </A>
        </li>
    }
}

#[component]
pub fn TagList<T: Fn() -> Vec<String> + 'static>(
    tags: T,
    #[prop(optional)] outline: bool,
) -> impl IntoView {
    view! {
        <ul class="tag-list">
            <For each=tags key=|tag| tag.clone() let:tag>
                <TagLink tag=tag outline=outline/>
            </For>
        </ul>
    }
}

#[server]
async fn popular_tags() -> Result<Vec<String>, ServerFnError> {
    sqlx::query_scalar!(
        "
        select tag from tag
        group by tag
        order by count(article) desc
        limit 10
        "
    )
    .fetch_all(crate::db::get())
    .await
    .map_err(|e| {
        tracing::error!("failed to get popular tags: {:?}", e);
        ServerFnError::ServerError("Could not get tags".into())
    })
}

/// Renders the home page of your application.
/// Expects a feed as nested route.
#[component]
fn HomePage() -> impl IntoView {
    let tags = create_resource(
        || (),
        |_| async { popular_tags().await.unwrap_or_default() },
    );
    view! {
        <div class="home-page">
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">conduit</h1>
                    <p>A place to share your knowledge.</p>
                </div>
            </div>

            <div class="container page">
                <div class="row">
                    <Outlet/>

                    <div class="col-md-3">
                        <div class="sidebar">
                            <p>Popular Tags</p>
                            <Suspense>
                                <TagList tags=move || tags().unwrap_or_default()/>
                            </Suspense>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Login(login: VoidAction<crate::auth::Login>) -> impl IntoView {
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
fn Register(register: VoidAction<crate::auth::Register>) -> impl IntoView {
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

#[derive(Params, PartialEq, Eq, Clone)]
struct UserParam {
    username: String,
}

#[component]
fn ProfileFeed(#[prop(optional)] favorites: bool) -> impl IntoView {
    let params = use_params::<UserParam>();
    let username = move || params().expect("username in path").username;
    let profile = move || profile_link(&username());
    let fav = move || format!("{}/favorites", profile());
    let kind = if favorites {
        Signal::derive(move || FeedKind::Favorited(username()))
    } else {
        Signal::derive(move || FeedKind::By(username()))
    };
    view! {
        <Feed kind=kind>
            <NavLink href=Signal::derive(profile)>My Articles</NavLink>
            <NavLink href=Signal::derive(fav)>Favorited Articles</NavLink>
        </Feed>
    }
}

#[server]
async fn profile_data(username: String) -> Result<Profile, ServerFnError> {
    let for_user = crate::auth::authenticated_username();
    User::profile(&username, for_user.as_deref())
        .await
        .map_err(|e| {
            tracing::error!("failed to get profile: {:?}", e);
            ServerFnError::ServerError("Could not fetch profile data".into())
        })
}

#[component]
pub fn ProfileImg(
    #[prop(into)] src: MaybeSignal<Option<String>>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    // TODO: default for missing images
    view! { <img src=move || src().unwrap_or_default() class=class/> }
}

#[component]
fn Profile() -> impl IntoView {
    let user = use_current_user();
    let params = use_params::<UserParam>();
    let username = move || params().expect("username in path").username;

    let profile = create_blocking_resource(username, profile_data);

    view! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <Transition fallback=|| "Loading profile...">
                            <ErrorBoundary fallback=error_boundary_fallback>
                                {move || {
                                    profile()
                                        .map(|p| {
                                            p.map(|p| {
                                                let p = create_rw_signal(p);
                                                view! {
                                                    <div class="col-xs-12 col-md-10 offset-md-1">
                                                        <ProfileImg
                                                            src=Signal::derive(move || p().image)
                                                            class="user-img"
                                                        />
                                                        <h4>{move || p().username}</h4>
                                                        <p>{move || p().bio}</p>
                                                        <Show
                                                            when=move || {
                                                                user.with(|u| {
                                                                    u.as_ref().is_some_and(|u| u.username == username())
                                                                })
                                                            }

                                                            fallback=move || {
                                                                view! {
                                                                    <FollowButton class="action-btn" profile=p.split()/>
                                                                }
                                                            }
                                                        >

                                                            <A
                                                                href="/settings"
                                                                class="btn btn-sm btn-outline-secondary action-btn"
                                                            >
                                                                <i class="ion-gear-a"></i>
                                                                {NBSP}
                                                                Edit Profile Settings
                                                            </A>
                                                        </Show>
                                                    </div>
                                                }
                                            })
                                        })
                                }}

                            </ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
            <div class="container">
                <div class="row">
                    <Outlet/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Settings(logout: VoidAction<crate::auth::Logout>) -> impl IntoView {
    view! {
        <div class="settings-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Your Settings</h1>

                        <ul class="error-messages">
                            <li>That name is required</li>
                        </ul>

                        <form>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="URL of profile picture"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Your Name"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control form-control-lg"
                                        rows="8"
                                        placeholder="Short bio about you"
                                    ></textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Email"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="New Password"
                                    />
                                </fieldset>
                                <button class="btn btn-lg btn-primary pull-xs-right">
                                    Update Settings
                                </button>
                            </fieldset>
                        </form>
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

#[server]
async fn toggle_follow(user: String, current: bool) -> Result<bool, ServerFnError> {
    let logged_in = crate::auth::require_login()?;
    if logged_in == user {
        // Can't follow oneself
        return Ok(false);
    }
    if current {
        sqlx::query!(
            "delete from follow where follower = ? and followed = ?",
            logged_in,
            user
        )
    } else {
        sqlx::query!(
            "insert or ignore into follow (follower, followed) values (?, ?)",
            logged_in,
            user
        )
    }
    .execute(crate::db::get())
    .await
    .map(|res| res.rows_affected() == 1)
    .map_err(|e| {
        tracing::error!("failed to toggle follow: {:?}", e);
        ServerFnError::ServerError("database error".into())
    })
}

#[server]
async fn toggle_favorite(article: String, current: bool) -> Result<bool, ServerFnError> {
    let logged_in = crate::auth::require_login()?;
    if sqlx::query_scalar!(
        "select author = ? from article where slug = ?",
        logged_in,
        article
    )
    .fetch_optional(crate::db::get())
    .await?
    .unwrap_or(1)
        != 0
    {
        // Can't favorite own article
        return Ok(false);
    }
    if current {
        sqlx::query!(
            "delete from favorite where user = ? and article = ?",
            logged_in,
            article
        )
    } else {
        sqlx::query!(
            "insert or ignore into favorite (user, article) values (?, ?)",
            logged_in,
            article
        )
    }
    .execute(crate::db::get())
    .await
    .map(|res| res.rows_affected() == 1)
    .map_err(|e| {
        tracing::error!("failed to toggle follow: {:?}", e);
        ServerFnError::ServerError("database error".into())
    })
}

#[derive(Params, PartialEq, Eq, Clone)]
pub(crate) struct ArticleSlugParam {
    pub slug: String,
}

#[component]
pub fn FavoriteButton(
    article: RwSignal<Article>,
    #[prop(optional)] compact: bool,
) -> impl IntoView {
    let user = use_current_user();
    let toggle = create_server_action::<ToggleFavorite>();
    let pending = toggle.pending();
    let result = toggle.value();
    let disabled = move || {
        with!(|user, article| {
            user.as_ref()
                .map_or(true, |user| user.username == article.author.username)
                || pending()
        })
    };
    let favorited = move || article.with(|a| a.favorited);

    create_effect(move |_| {
        let success = result.with(|res| matches!(res, Some(Ok(true))));
        if success {
            article.update(|a| {
                if a.favorited {
                    a.favorited = false;
                    a.favorites_count -= 1;
                } else {
                    a.favorited = true;
                    a.favorites_count += 1;
                }
            });
        }
    });

    let text = if compact { "" } else { "Favorite article" };

    view! {
        <ActionForm action=toggle>
            <button type="submit" disabled=disabled class="btn btn-sm btn-outline-primary">
                <i class="ion-heart"></i>
                {NBSP}
                {text}
                <span class="counter">"(" {move || article.with(|a| a.favorites_count)} ")"</span>
            </button>
            <input type="hidden" name="article" value=move || article.with(|a| a.slug.clone())/>
            <input type="hidden" name="current" value=move || favorited().to_string()/>
        </ActionForm>
    }
}

// Bit annoying to work around different ways signals can be paired and split.
// Slice has different type to RwSignal::split and there's no (proper) way to join the pairs back.
// Might improve, see: https://github.com/leptos-rs/leptos/discussions/2356
#[component]
pub fn FollowButton<R: Fn() -> Profile + 'static + Copy, W: Fn(Profile) + 'static>(
    #[prop(optional)] class: &'static str,
    profile: (R, W),
) -> impl IntoView {
    let (profile, set_profile) = profile;
    let toggle = create_server_action::<ToggleFollow>();
    let result = toggle.value();
    let user = move || profile().username;

    create_effect(move |_| {
        let success = result.with(|res| matches!(res, Some(Ok(true))));
        if success {
            // Note: bit awkward to work with slices
            let mut p = profile();
            p.following = !p.following;
            set_profile(p);
        }
    });

    let follow = create_memo(move |_| {
        if profile().following {
            ("Unfollow", "ion-minus-round")
        } else {
            ("Follow", "ion-plus-round")
        }
    });
    let class = format!("btn btn-sm btn-outline-secondary {}", class);

    view! {
        <ActionForm action=toggle>
            <button type="submit" disabled=toggle.pending() class=class>
                <i class=move || follow().1></i>
                {NBSP}
                {move || follow().0}
                {NBSP}
                {user}
            </button>
            <input type="hidden" name="user" value=user/>
            <input type="hidden" name="current" value=move || profile().following.to_string()/>

        </ActionForm>
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
enum FeedKind {
    Feed,
    Global,
    By(String),
    Favorited(String),
    Tag(String),
}

#[server]
async fn get_feed(kind: FeedKind) -> Result<Feed, ServerFnError> {
    use crate::models::article::FeedOptions;

    let user = "noone"; // TODO: current user
    let options = FeedOptions::default();
    match kind {
        FeedKind::Feed => Feed::feed(user, &options).await,
        FeedKind::Global => Feed::global(&options).await,
        FeedKind::By(user) => Feed::by(&user, &options).await,
        FeedKind::Favorited(user) => Feed::favorited(&user, &options).await,
        FeedKind::Tag(tag) => Feed::tag(&tag, &options).await,
    }
    .map_err(|e| {
        tracing::error!("sql error when fetching feed: {:?}", e);
        ServerFnError::ServerError("Could not fetch feed".into())
    })
}

pub fn profile_link(username: &str) -> String {
    format!("/profile/{}", username)
}

#[component]
fn Feed(#[prop(into)] kind: MaybeSignal<FeedKind>, children: Children) -> impl IntoView {
    let feed = create_resource(kind, get_feed);
    view! {
        <div class="col-md-9">
            <div class="feed-toggle">
                <ul class="nav nav-pills outline-active">{children()}</ul>
            </div>

            // TODO: Maybe try `Transition`
            <Suspense fallback=|| "Loading feed...">
                <ErrorBoundary fallback=error_boundary_fallback>
                    {move || {
                        feed()
                            .map(|data| {
                                data.map(|articles| {
                                    view! {
                                        <For
                                            each=move || articles.articles.clone()
                                            key=|article| article.slug.clone()
                                            let:article
                                        >
                                            <article::Preview article=create_rw_signal(
                                                article,
                                            )></article::Preview>
                                        </For>
                                    }
                                })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>

            // TODO
            <ul class="pagination">
                <li class="page-item active">
                    <a class="page-link" href="">
                        1
                    </a>
                </li>
                <li class="page-item">
                    <a class="page-link" href="">
                        2
                    </a>
                </li>
            </ul>
        </div>
    }
}
