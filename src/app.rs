#![allow(clippy::empty_docs)]

use crate::{
    error_template::{error_boundary_fallback, AppError, ErrorTemplate},
    models::{
        article::{Article, Feed},
        comment::Comment,
        user::{Profile, User},
    },
};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};

fn use_current_user() -> Signal<Option<User>> {
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
        <Router
            trailing_slash=TrailingSlash::Redirect
            fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }
        >

            <header>
                <Suspense>
                    <Nav/>
                </Suspense>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=HomePage>
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
                    <Route path="/profile/:username" view=|| view! { <Profile/> }/>
                    <Route
                        path="/profile/:username/favorites"
                        view=|| view! { <Profile favorites=true/> }
                    />
                    <Route path="/article/:slug" view=Article/>
                    // TODO: this fails with TrailingSlash::Redirect
                    // <Route path="/editor/:slug?" view=Editor/>
                    <Route path="/editor" view=Editor/>
                    <Route path="/editor/:slug" view=Editor/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

#[component]
fn NavLink(#[prop(into)] href: MaybeSignal<String>, children: Children) -> impl IntoView {
    let class = {
        let path = use_location().pathname;
        let href = href.clone();
        move || {
            href.with(|href| {
                let active = href.is_empty()
                    || path.with(|path| {
                        if href == "/" {
                            path == "/"
                        } else {
                            path.starts_with(href)
                        }
                    });
                if active {
                    // TODO: see if `aria-current` could be used for the `active` class,
                    // since the <A> tag should set that for links.
                    "nav-link active"
                } else {
                    "nav-link"
                }
            })
        }
    };
    view! {
        <li class="nav-item">
            <A class=class href=href>
                {children()}
            </A>
        </li>
    }
}

const NBSP: &str = "\u{A0}";

#[component]
fn Nav() -> impl IntoView {
    let user = use_current_user();
    create_effect(move |_| {
        logging::log!("{:?}", user());
    });
    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <a class="navbar-brand" href="/">
                    conduit
                </a>
                <ul class="nav navbar-nav pull-xs-right">
                    <NavLink href="/">Home</NavLink>
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
                                        <NavLink href=profile_link(
                                            &user.username,
                                        )>
                                            {user
                                                .image
                                                .map(|img| {
                                                    view! { <img src=img class="user-pic"/> }
                                                })}
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
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
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
fn TagList(
    #[prop(into)] tags: MaybeSignal<Vec<String>>,
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

/// Renders the home page of your application.
/// Expects a feed as nested route.
#[component]
fn HomePage() -> impl IntoView {
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
                    <Suspense fallback=|| "Loading feed...">
                        <Outlet/>
                    </Suspense>

                    <div class="col-md-3">
                        <div class="sidebar">
                            <p>Popular Tags</p>

                            <TagList tags=vec![
                                "programming".into(),
                                "javascript".into(),
                                "emberjs".into(),
                                "angularjs".into(),
                                "react".into(),
                                "mean".into(),
                                "node".into(),
                                "rails".into(),
                            ]/>
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
fn Profile(#[prop(optional)] favorites: bool) -> impl IntoView {
    let params = use_params::<UserParam>();
    view! {
        <div class="profile-page">
            <div class="user-info">
                <div class="container">
                    <div class="row">
                        <div class="col-xs-12 col-md-10 offset-md-1">
                            <img src="http://i.imgur.com/Qr71crq.jpg" class="user-img"/>
                            <h4>Eric Simons</h4>
                            <p>
                                "Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from the Hunger Games"
                            </p>
                            <button class="btn btn-sm btn-outline-secondary action-btn">
                                <i class="ion-plus-round"></i>
                                {NBSP}
                                Follow Eric Simons
                            </button>
                            <button class="btn btn-sm btn-outline-secondary action-btn">
                                <i class="ion-gear-a"></i>
                                {NBSP}
                                Edit Profile Settings
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container">
                <div class="row">
                    {move || {
                        params()
                            .map(|author| {
                                view! {
                                    // class="col-xs-12 col-md-10 offset-md-1"
                                    <Feed kind=if favorites {
                                        FeedKind::Favorited(author.username)
                                    } else {
                                        FeedKind::By(author.username)
                                    }>
                                        <NavLink href=if favorites {
                                            ".."
                                        } else {
                                            ""
                                        }>My Articles</NavLink>
                                        <NavLink href=if favorites {
                                            ""
                                        } else {
                                            "favorites"
                                        }>Favorited Articles</NavLink>
                                    </Feed>
                                }
                            })
                    }}

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

#[component]
fn Editor() -> impl IntoView {
    view! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ul class="error-messages">
                            <li>That title is required</li>
                        </ul>

                        <form>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control form-control-lg"
                                        placeholder="Article Title"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="What's this article about?"
                                    />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in markdown)"
                                    ></textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        type="text"
                                        class="form-control"
                                        placeholder="Enter tags"
                                    />
                                    // TODO: client side fancy stuff for tags
                                    <div class="tag-list">
                                        <span class="tag-default tag-pill">
                                            <i class="ion-close-round"></i>
                                            tag
                                        </span>
                                    </div>
                                </fieldset>
                                <button class="btn btn-lg pull-xs-right btn-primary" type="button">
                                    Publish Article
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[server]
#[tracing::instrument]
async fn toggle_follow(user: String, current: bool) -> Result<bool, ServerFnError> {
    let logged_in = crate::auth::require_login()?;
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
#[tracing::instrument]
async fn toggle_favorite(article: String, current: bool) -> Result<bool, ServerFnError> {
    let logged_in = crate::auth::require_login()?;
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

#[component]
fn ArticleActions(#[prop(into)] article: Signal<Article>) -> impl IntoView {
    // TODO: use reactive Article prop
    let user = use_current_user();
    let is_logged_in = move || user.with(Option::is_some);
    let author = Signal::derive(move || article.with(|a| a.author.username.clone()));
    let is_author = Signal::derive(move || {
        user.with(|user| user.as_ref().is_some_and(|user| user.username == author()))
    });

    let toggle_follow = create_server_action::<ToggleFollow>();
    let toggle_favorite = create_server_action::<ToggleFavorite>();

    view! {
        <ArticleMeta article=article>
            <Suspense>
                <Show
                    when=is_author
                    fallback=move || {
                        view! {
                            <Show when=is_logged_in>
                                <ActionForm action=toggle_follow>
                                    <button
                                        type="submit"
                                        disabled=toggle_follow.pending()
                                        class="btn btn-sm btn-outline-secondary"
                                    >
                                        <i class="ion-plus-round"></i>
                                        {NBSP}
                                        Follow
                                        {author}
                                    </button>
                                    <input type="hidden" name="user" value=author/>
                                    <input
                                        type="hidden"
                                        name="current"
                                        value=move || {
                                            article.with(|a| a.author.following).to_string()
                                        }
                                    />
                                </ActionForm>
                                {NBSP}
                                <ActionForm action=toggle_favorite>
                                    <button
                                        type="submit"
                                        disabled=toggle_favorite.pending()
                                        class="btn btn-sm btn-outline-primary"
                                    >
                                        <i class="ion-heart"></i>
                                        {NBSP}
                                        Favorite Article
                                        <span class="counter">
                                            "(" {move || article.with(|a| a.favorites_count)} ")"
                                        </span>
                                    </button>
                                    <input
                                        type="hidden"
                                        name="article"
                                        value=move || article.with(|a| a.slug.clone())
                                    />
                                    <input
                                        type="hidden"
                                        name="current"
                                        value=move || article.with(|a| a.favorited).to_string()
                                    />
                                </ActionForm>
                            </Show>
                        }
                    }
                >

                    <button class="btn btn-sm btn-outline-secondary">
                        <i class="ion-edit"></i>
                        Edit Article
                    </button>
                    {NBSP}
                    <button class="btn btn-sm btn-outline-danger">
                        <i class="ion-trash-a"></i>
                        Delete Article
                    </button>
                </Show>
            </Suspense>
        </ArticleMeta>
    }
}

#[component]
fn Article() -> impl IntoView {
    // TODO: update to switch between follow/favorite AND edit/delete
    let [article, _] = placeholder_articles();
    let (article, _) = create_signal(article);
    view! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>{move || article.with(|a| a.title.clone())}</h1>

                    <ArticleActions article=article/>
                </div>
            </div>

            <div class="container page">
                <div class="row article-content">
                    <div class="col-md-12">
                        // TODO: This is a bit of a hack, but let's roll with it for now
                        <div id="content" style="all: initial">
                            <pre>{move || article.with(|a| a.body.clone())}</pre>
                            <div></div>
                        </div>
                        <script type="module">
                            "
                                import { marked } from 'https://cdn.jsdelivr.net/npm/marked/lib/marked.esm.js';
                                import DOMPurify from 'https://cdn.jsdelivr.net/npm/dompurify@3.1.0/+esm'
                                const [pre, target] = document.getElementById('content').children;
                                pre.style.display = 'none';
                                target.innerHTML = DOMPurify.sanitize(marked.parse(pre.textContent));
                            "
                        </script>
                        <TagList
                            outline=true
                            tags=Signal::derive(move || article.with(|a| a.tags.clone()))
                        />
                    </div>
                </div>

                <hr/>

                <div class="article-actions">
                    <ArticleActions article=article/>
                </div>

                <div class="row">
                    <Comments article_slug=Signal::derive(move || {
                        article.with(|a| a.slug.clone())
                    })/>
                </div>
            </div>
        </div>
    }
}

#[server]
async fn comments(slug: String) -> Result<Vec<Comment>, ServerFnError> {
    Comment::for_article(&slug).await.map_err(|e| {
        tracing::error!("failed to fetch comments: {:?}", e);
        ServerFnError::ServerError("Failed to fetch comments".into())
    })
}

#[component]
fn CommentCard(comment: Comment) -> impl IntoView {
    let link = profile_link(&comment.author.username);
    view! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{&comment.body}</p>
            </div>
            <div class="card-footer">
                <A href=link.clone() class="comment-author">
                    {comment
                        .author
                        .image
                        .map(|url| {
                            view! { <img src=url class="comment-author-img"/> }
                        })}

                </A>
                {NBSP}
                <A href=link class="comment-author">
                    {&comment.author.username}
                </A>
                <span class="date-posted">{&comment.created_at}</span>
            </div>
        </div>
    }
}

#[component]
fn Comments(#[prop(into)] article_slug: MaybeSignal<String>) -> impl IntoView {
    let comments = create_resource(article_slug, comments);
    view! {
        <div class="col-xs-12 col-md-8 offset-md-2">
            <form class="card comment-form">
                <div class="card-block">
                    <textarea
                        class="form-control"
                        placeholder="Write a comment..."
                        rows="3"
                    ></textarea>
                </div>
                <div class="card-footer">
                    <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img"/>
                    <button class="btn btn-sm btn-primary">Post Comment</button>
                </div>
            </form>

            // TODO: Maybe try `Transition`
            <Suspense fallback=move || {
                view! { <p>"Loading comments..."</p> }
            }>
                {move || {
                    comments()
                        .map(move |data| {
                            view! {
                                <ErrorBoundary fallback=error_boundary_fallback>

                                    {data
                                        .map(|comments| {
                                            view! {
                                                <For
                                                    each=move || comments.clone()
                                                    key=|comment| comment.id
                                                    let:comment
                                                >
                                                    <CommentCard comment=comment/>
                                                </For>
                                            }
                                        })}

                                </ErrorBoundary>
                            }
                        })
                }}

            </Suspense>
        </div>
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

fn placeholder_authors() -> [Profile; 2] {
    [
        Profile {
            username: "eric-simons".into(),
            bio: None,
            image: Some("http://i.imgur.com/Qr71crq.jpg".into()),
            following: false,
        },
        Profile {
            username: "albert-pai".into(),
            bio: None,
            image: Some("http://i.imgur.com/N4VcUeJ.jpg".into()),
            following: false,
        },
    ]
}

fn placeholder_articles() -> [Article; 2] {
    let [first, second] = placeholder_authors();
    [
        Article {
            slug: "how-to-build-webapps-that-scale".into(),
            title: "How to build webapps that scale".into(),
            description: "This is the description for the post.".into(),
            body: "\
# Header

this is some content

- list 1
- list 2
- list 3

"
            .into(),
            tags: vec!["realworld".into(), "implementations".into()],
            created_at: "January 20th".into(),
            updated_at: None,
            favorited: false,
            favorites_count: 29,
            author: first,
        },
        Article {
            slug: "the-song-you".into(),
            title: "The song you won't ever stop singing. No matter how hard you try.".into(),
            description: "This is the description for the post.".into(),
            body: "".into(),
            tags: vec![
                "realworld".into(),
                "implementations".into(),
                "one-more".into(),
            ],
            created_at: "January 20th".into(),
            updated_at: None,
            favorited: false,
            favorites_count: 32,
            author: second,
        },
    ]
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

fn profile_link(username: &str) -> String {
    format!("/profile/{}", username)
}

fn format_date(date: &str) -> String {
    date.to_string()
}

#[component]
fn ArticleMeta(#[prop(into)] article: Signal<Article>, children: Children) -> impl IntoView {
    let author_link = move || article.with(|a| profile_link(&a.author.username));
    view! {
        <div
            class="article-meta"
            style="display: flex; flex-direction: row; justify-content: center"
        >
            <A href=author_link>
                {move || {
                    article.with(|a| a.author.image.as_ref().map(|url| view! { <img src=url/> }))
                }}

            </A>
            <div class="info">
                <A href=author_link class="author">
                    {move || article.with(|a| a.author.username.clone())}
                </A>
                <span class="date">{move || article.with(|a| format_date(&a.created_at))}</span>
                {move || {
                    article
                        .with(|a| {
                            a.updated_at
                                .as_ref()
                                .map(|updated| {
                                    view! { <span class="date">{format_date(updated)}</span> }
                                })
                        })
                }}

            </div>
            {children()}
        </div>
    }
}

#[component]
fn ArticlePreview(#[prop(into)] article: Signal<Article>) -> impl IntoView {
    let article_link = move || article.with(|a| format!("/article/{}", a.slug));
    view! {
        <div class="article-preview">
            <ArticleMeta article=article>
                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                    <i class="ion-heart"></i>
                    {move || article.with(|a| a.favorites_count)}
                </button>
            </ArticleMeta>
            <A href=article_link class="preview-link">
                <h1>{move || article.with(|a| a.title.clone())}</h1>
                <p>{move || article.with(|a| a.description.clone())}</p>
                <span>Read more...</span>
                <TagList
                    outline=true
                    tags=Signal::derive(move || article.with(|a| a.tags.clone()))
                />
            </A>
        </div>
    }
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
            <Suspense fallback=|| {
                view! { <p>"Loading feed..."</p> }
            }>
                {move || {
                    feed()
                        .map(move |data| {
                            view! {
                                <ErrorBoundary fallback=error_boundary_fallback>

                                    {data
                                        .map(|articles| {
                                            view! {
                                                <For
                                                    each=move || articles.articles.clone()
                                                    key=|article| article.slug.clone()
                                                    let:article
                                                >
                                                    <ArticlePreview article=create_rw_signal(article)/>
                                                </For>
                                            }
                                        })}

                                </ErrorBoundary>
                            }
                        })
                }}

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
