#![allow(clippy::empty_docs)]

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
                <Nav/>
            </header>
            <main>
                <Routes>
                    <Route path="" view=HomePage>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <Feed kind=FeedKind::Global>
                                        <NavLink href="/feed">Your Feed</NavLink>
                                        <NavLink href="" active=true>
                                            Global Feed
                                        </NavLink>
                                    </Feed>
                                }
                            }
                        />

                        <Route
                            path="/feed"
                            view=|| {
                                view! {
                                    <Feed kind=FeedKind::Feed>
                                        <NavLink href="/feed" active=true>
                                            Your Feed
                                        </NavLink>
                                        <NavLink href="/">Global Feed</NavLink>
                                    </Feed>
                                }
                            }
                        />

                        <Route
                            path="/tag/:tag"
                            view=|| {
                                let params = use_params_map();
                                let tag = move || {
                                    params.with(|map| map.get("tag").cloned().unwrap_or_default())
                                };
                                view! {
                                    <Feed kind=FeedKind::Tag(tag())>
                                        <NavLink href="/feed">Your Feed</NavLink>
                                        <NavLink href="/">Global Feed</NavLink>
                                        <NavLink href="" active=true>
                                            #
                                            {tag}
                                        </NavLink>
                                    </Feed>
                                }
                            }
                        />

                    </Route>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/settings" view=Settings/>
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
fn NavLink(
    href: &'static str,
    #[prop(optional)] active: bool,
    children: Children,
) -> impl IntoView {
    let link_class = if active {
        "nav-link active"
    } else {
        "nav-link"
    };
    view! {
        <li class="nav-item">
            <A class=link_class href=href>
                {children()}
            </A>
        </li>
    }
}

const NBSP: &str = "\u{A0}";

#[component]
fn Nav() -> impl IntoView {
    let authenticated = false; // TODO
    if authenticated {
        view! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <a class="navbar-brand" href="/">
                        conduit
                    </a>
                    <ul class="nav navbar-nav pull-xs-right">
                        // Add "active" class when you're on that page
                        <NavLink href="/" active=true>
                            Home
                        </NavLink>
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
                        <NavLink href="/profile/eric-simons">
                            <img src="" class="user-pic"/>
                            Eric Simons
                        </NavLink>
                    </ul>
                </div>
            </nav>
        }
    } else {
        view! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <a class="navbar-brand" href="/">
                        conduit
                    </a>
                    <ul class="nav navbar-nav pull-xs-right">
                        // Add "active" class when you're on that page
                        <NavLink href="/" active=true>
                            Home
                        </NavLink>
                        <NavLink href="/login">Sign in</NavLink>
                        <NavLink href="/register">Sign up</NavLink>
                    </ul>
                </div>
            </nav>
        }
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

/// Renders the home page of your application.
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
                    <Outlet/>

                    <div class="col-md-3">
                        <div class="sidebar">
                            <p>Popular Tags</p>

                            <div class="tag-list">
                                <a href="" class="tag-pill tag-default">
                                    programming
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    javascript
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    emberjs
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    angularjs
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    react
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    mean
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    node
                                </a>
                                <a href="" class="tag-pill tag-default">
                                    rails
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign in</h1>
                        <p class="text-xs-center">
                            <a href="/register">Need an account?</a>
                        </p>

                        <ul class="error-messages">
                            <li>That email is already taken</li>
                        </ul>

                        <form>
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
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button class="btn btn-lg btn-primary pull-xs-right">Sign in</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Register() -> impl IntoView {
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

                        <form>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Username"
                                />
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
                                    placeholder="Password"
                                />
                            </fieldset>
                            <button class="btn btn-lg btn-primary pull-xs-right">Sign up</button>
                        </form>
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
                                        <NavLink
                                            href=if favorites { ".." } else { "" }
                                            active=!favorites
                                        >
                                            My Articles
                                        </NavLink>
                                        <NavLink
                                            href=if favorites { "" } else { "favorites" }
                                            active=favorites
                                        >
                                            Favorited Articles
                                        </NavLink>
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
fn Settings() -> impl IntoView {
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
                        <button class="btn btn-outline-danger">Or click here to logout.</button>
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

#[component]
fn Article() -> impl IntoView {
    // TODO: update to switch between follow/favorite AND edit/delete
    let [article, _] = placeholder_articles();
    view! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>How to build webapps that scale</h1>

                    <ArticleMeta article=article.clone()>
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-plus-round"></i>
                            {NBSP}
                            Follow Eric Simons
                            <span class="counter">(10)</span>
                        </button>
                        {NBSP}
                        {NBSP}
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i>
                            {NBSP}
                            Favorite Post
                            <span class="counter">(29)</span>
                        </button>
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-edit"></i>
                            Edit Article
                        </button>
                        <button class="btn btn-sm btn-outline-danger">
                            <i class="ion-trash-a"></i>
                            Delete Article
                        </button>
                    </ArticleMeta>
                </div>
            </div>

            <div class="container page">
                <div class="row article-content">
                    <div class="col-md-12">
                        <p>
                            Web development technologies have evolved at an incredible clip over the past few years.
                        </p>
                        <h2 id="introducing-ionic">Introducing RealWorld.</h2>
                        <p>"It's a great solution for learning how other frameworks work."</p>
                        <ul class="tag-list">
                            <li class="tag-default tag-pill tag-outline">realworld</li>
                            <li class="tag-default tag-pill tag-outline">implementations</li>
                        </ul>
                    </div>
                </div>

                <hr/>

                <div class="article-actions">
                    <ArticleMeta article=article.clone()>
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-plus-round"></i>
                            {NBSP}
                            Follow Eric Simons
                        </button>
                        {NBSP}
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i>
                            {NBSP}
                            Favorite Article
                            <span class="counter">(29)</span>
                        </button>
                        <button class="btn btn-sm btn-outline-secondary">
                            <i class="ion-edit"></i>
                            Edit Article
                        </button>
                        <button class="btn btn-sm btn-outline-danger">
                            <i class="ion-trash-a"></i>
                            Delete Article
                        </button>
                    </ArticleMeta>
                </div>

                <div class="row">
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
                                <img
                                    src="http://i.imgur.com/Qr71crq.jpg"
                                    class="comment-author-img"
                                />
                                <button class="btn btn-sm btn-primary">Post Comment</button>
                            </div>
                        </form>

                        <div class="card">
                            <div class="card-block">
                                <p class="card-text">
                                    With supporting text below as a natural lead-in to additional content.
                                </p>
                            </div>
                            <div class="card-footer">
                                <a href="/profile/author" class="comment-author">
                                    <img
                                        src="http://i.imgur.com/Qr71crq.jpg"
                                        class="comment-author-img"
                                    />
                                </a>
                                {NBSP}
                                <a href="/profile/jacob-schmidt" class="comment-author">
                                    Jacob Schmidt
                                </a>
                                <span class="date-posted">Dec 29th</span>
                            </div>
                        </div>

                        <div class="card">
                            <div class="card-block">
                                <p class="card-text">
                                    With supporting text below as a natural lead-in to additional content.
                                </p>
                            </div>
                            <div class="card-footer">
                                <a href="/profile/author" class="comment-author">
                                    <img
                                        src="http://i.imgur.com/Qr71crq.jpg"
                                        class="comment-author-img"
                                    />
                                </a>
                                {NBSP}
                                <a href="/profile/jacob-schmidt" class="comment-author">
                                    Jacob Schmidt
                                </a>
                                <span class="date-posted">Dec 29th</span>
                                <span class="mod-options">
                                    <i class="ion-trash-a"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(default)]
    pub following: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    // TODO: consider using proper date type
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: u32,
    pub author: Author,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Articles {
    pub articles: Vec<Article>,
    /// The full number of articles in the requested feed
    pub articles_count: u32,
}

fn placeholder_authors() -> [Author; 2] {
    [
        Author {
            username: "eric-simons".into(),
            bio: None,
            image: Some("http://i.imgur.com/Qr71crq.jpg".into()),
            following: false,
        },
        Author {
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
            body: "".into(),
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
async fn get_feed(kind: FeedKind) -> Result<Articles, ServerFnError> {
    match kind {
        FeedKind::Feed => (),
        FeedKind::Global => (),
        FeedKind::By(_) => (),
        FeedKind::Favorited(_) => (),
        FeedKind::Tag(_) => (),
    }

    let articles = placeholder_articles().to_vec();
    let articles_count = articles.len() as u32;
    Ok(Articles {
        articles,
        articles_count,
    })
}

fn format_date(date: &str) -> String {
    date.to_string()
}

#[component]
fn ArticleMeta(article: Article, children: Children) -> impl IntoView {
    let author_link = format!("/profile/{}", article.author.username);
    view! {
        <div class="article-meta">
            <A href=author_link
                .clone()>{article.author.image.map(|url| view! { <img src=url/> })}</A>
            <div class="info">
                <A href=author_link class="author">
                    {article.author.username}
                </A>
                <span class="date">{format_date(&article.created_at)}</span>
                {article
                    .updated_at
                    .map(|updated| view! { <span class="date">{format_date(&updated)}</span> })}

            </div>
            {children()}
        </div>
    }
}

#[component]
fn ArticlePreview(article: Article) -> impl IntoView {
    let article_link = format!("/article/{}", article.slug);
    view! {
        <div class="article-preview">
            <ArticleMeta article=article.clone()>
                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                    <i class="ion-heart"></i>
                    {article.favorites_count}
                </button>
            </ArticleMeta>
            <A href=article_link class="preview-link">
                <h1>{article.title}</h1>
                <p>{article.description}</p>
                <span>Read more...</span>
                <ul class="tag-list">
                    {article
                        .tags
                        .iter()
                        .map(|tag| {
                            view! { <li class="tag-default tag-pill tag-outline">{tag}</li> }
                        })
                        .collect_view()}

                </ul>
            </A>
        </div>
    }
}

#[component]
fn Feed(kind: FeedKind, children: Children) -> impl IntoView {
    let feed = create_resource(move || kind.clone(), get_feed);
    view! {
        <div class="col-md-9">
            <div class="feed-toggle">
                <ul class="nav nav-pills outline-active">{children()}</ul>
            </div>

            // TODO: Add error boundary
            // Maybe try `Transition`
            <Suspense fallback=move || {
                view! { <p>"Loading feed..."</p> }
            }>
                {move || {
                    feed()
                        .map(move |data| {
                            view! {
                                <ErrorBoundary fallback=|errors| {
                                    view! { <ErrorTemplate errors=errors/> }
                                }>

                                    {data
                                        .map(|articles| {
                                            view! {
                                                <For
                                                    each=move || articles.articles.clone()
                                                    key=|article| article.slug.clone()
                                                    let:article
                                                >
                                                    <ArticlePreview article=article/>
                                                </For>
                                            }
                                        })}

                                </ErrorBoundary>
                            }
                        })
                }}

            </Suspense>

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
