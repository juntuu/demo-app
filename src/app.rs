#![allow(clippy::empty_docs)]

// TODO:
// - some validation for user details

use crate::{
    error_template::{AppError, ErrorTemplate},
    models::user::{Profile, User},
    pages::{
        article::Article,
        editor,
        feed::{Feed, FeedKind},
        profile::{profile_link, ProfileImg, ProfileRoute},
        user::{Login, Register, Settings},
    },
};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::*;

pub(crate) fn use_current_user() -> Signal<Option<User>> {
    expect_context()
}

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
                                <Nav logout=logout/>
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
                                        <UserFeedLink href="/feed"/>
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
                                        <UserFeedLink href=""/>
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
                                        <UserFeedLink href="/feed"/>
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
                    <ProfileRoute/>
                    <Route path="/article/:slug" view=Article/>
                    <Route path="/editor" view=editor::New/>
                    <Route path="/editor/:slug" view=editor::Edit/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
pub fn UserFeedLink(href: &'static str) -> impl IntoView {
    let user = use_current_user();
    view! {
        <Suspense>
            {move || {
                user.with(|u| u.as_ref().map(|_| view! { <NavLink href=href>Your Feed</NavLink> }))
            }}

        </Suspense>
    }
}

#[component]
pub fn NavLink(#[prop(into)] href: MaybeSignal<String>, children: Children) -> impl IntoView {
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
fn Nav(logout: crate::auth::LogoutAction) -> impl IntoView {
    let user = use_current_user();
    let links = move || {
        if let Some(user) = user() {
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
                <li class="nav-item">
                    <ActionForm action=logout>
                        {NBSP} <button type="submit" class="btn btn-outline-danger">
                            Logout
                        </button>
                    </ActionForm>
                </li>
            }
        } else {
            view! {
                <NavLink href="/login">Sign in</NavLink>
                <NavLink href="/register">Sign up</NavLink>
            }
        }
    };

    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <A class="navbar-brand" href="/">
                    conduit
                </A>
                <ul class="nav navbar-nav pull-xs-right">
                    <NavLink href="/">Home</NavLink>
                    <Suspense>{links}</Suspense>
                </ul>
            </div>
        </nav>
    }
}

#[component]
pub fn TagList<T: Fn() -> Vec<String> + 'static>(
    tags: T,
    #[prop(optional)] outline: bool,
) -> impl IntoView {
    let class = if outline {
        "tag-pill tag-default tag-outline"
    } else {
        "tag-pill tag-default"
    };
    view! {
        <ul class="tag-list">
            <For each=tags key=|tag| tag.clone() let:tag>
                <li>
                    <A href=format!("/tag/{}", &tag) class=class>
                        {tag}
                    </A>
                </li>
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
                            <Suspense fallback=|| "Loading...">
                                <TagList tags=move || tags().unwrap_or_default()/>
                            </Suspense>
                        </div>
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

#[derive(Params, PartialEq, Eq, Clone)]
pub(crate) struct ArticleSlugParam {
    pub slug: String,
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
