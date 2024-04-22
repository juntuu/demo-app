#![allow(clippy::empty_docs)]

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

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
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Nav/>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/settings" view=Settings/>
                    <Route path="/profile/:username" view=|| view! { <Profile/> }/>
                    <Route path="/profile/:username/favorites" view=|| view! { <Profile favorites=true/> }/>
                    <Route path="/article/:slug" view=Article/>
                    <Route path="/editor/:slug?" view=Editor/>
                </Routes>
                <Footer/>
            </main>
        </Router>
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
                        <li class="nav-item">
                            // Add "active" class when you're on that page
                            <a class="nav-link active" href="/">
                                Home
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/editor">
                                <i class="ion-compose"></i>
                                {NBSP}
                                New Article
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/settings">
                                <i class="ion-gear-a"></i>
                                {NBSP}
                                Settings
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/profile/eric-simons">
                                <img src="" class="user-pic"/>
                                Eric Simons
                            </a>
                        </li>
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
                        <li class="nav-item">
                            // Add "active" class when you're on that page
                            <a class="nav-link active" href="/">
                                Home
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/login">
                                Sign in
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/register">
                                Sign up
                            </a>
                        </li>
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
                    <div class="col-md-9">
                        <div class="feed-toggle">
                            <ul class="nav nav-pills outline-active">
                                <li class="nav-item">
                                    <a class="nav-link" href="">
                                        Your Feed
                                    </a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link active" href="">
                                        Global Feed
                                    </a>
                                </li>
                            </ul>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href="/profile/eric-simons">
                                    <img src="http://i.imgur.com/Qr71crq.jpg"/>
                                </a>
                                <div class="info">
                                    <a href="/profile/eric-simons" class="author">
                                        Eric Simons
                                    </a>
                                    <span class="date">January 20th</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i>
                                    29
                                </button>
                            </div>
                            <a href="/article/how-to-build-webapps-that-scale" class="preview-link">
                                <h1>How to build webapps that scale</h1>
                                <p>This is the description for the post.</p>
                                <span>Read more...</span>
                                <ul class="tag-list">
                                    <li class="tag-default tag-pill tag-outline">realworld</li>
                                    <li class="tag-default tag-pill tag-outline">
                                        implementations
                                    </li>
                                </ul>
                            </a>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href="/profile/albert-pai">
                                    <img src="http://i.imgur.com/N4VcUeJ.jpg"/>
                                </a>
                                <div class="info">
                                    <a href="/profile/albert-pai" class="author">
                                        Albert Pai
                                    </a>
                                    <span class="date">January 20th</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i>
                                    32
                                </button>
                            </div>
                            <a href="/article/the-song-you" class="preview-link">
                                <h1>
                                    "The song you won't ever stop singing. No matter how hard you try."
                                </h1>
                                <p>This is the description for the post.</p>
                                <span>Read more...</span>
                                <ul class="tag-list">
                                    <li class="tag-default tag-pill tag-outline">realworld</li>
                                    <li class="tag-default tag-pill tag-outline">
                                        implementations
                                    </li>
                                </ul>
                            </a>
                        </div>

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

#[component]
fn Profile(#[prop(default = false)] favorites: bool) -> impl IntoView {
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
                    <div class="col-xs-12 col-md-10 offset-md-1">
                        <div class="articles-toggle">
                            <ul class="nav nav-pills outline-active">
                                <li class="nav-item">
                                    <a class="nav-link active" href="">
                                        My Articles
                                    </a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="">
                                        Favorited Articles
                                    </a>
                                </li>
                            </ul>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href="/profile/eric-simons">
                                    <img src="http://i.imgur.com/Qr71crq.jpg"/>
                                </a>
                                <div class="info">
                                    <a href="/profile/eric-simons" class="author">
                                        Eric Simons
                                    </a>
                                    <span class="date">January 20th</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i>
                                    29
                                </button>
                            </div>
                            <a href="/article/how-to-buil-webapps-that-scale" class="preview-link">
                                <h1>How to build webapps that scale</h1>
                                <p>This is the description for the post.</p>
                                <span>Read more...</span>
                                <ul class="tag-list">
                                    <li class="tag-default tag-pill tag-outline">realworld</li>
                                    <li class="tag-default tag-pill tag-outline">
                                        implementations
                                    </li>
                                </ul>
                            </a>
                        </div>

                        <div class="article-preview">
                            <div class="article-meta">
                                <a href="/profile/albert-pai">
                                    <img src="http://i.imgur.com/N4VcUeJ.jpg"/>
                                </a>
                                <div class="info">
                                    <a href="/profile/albert-pai" class="author">
                                        Albert Pai
                                    </a>
                                    <span class="date">January 20th</span>
                                </div>
                                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                    <i class="ion-heart"></i>
                                    32
                                </button>
                            </div>
                            <a href="/article/the-song-you" class="preview-link">
                                <h1>
                                    "The song you won't ever stop singing. No matter how hard you try."
                                </h1>
                                <p>This is the description for the post.</p>
                                <span>Read more...</span>
                                <ul class="tag-list">
                                    <li class="tag-default tag-pill tag-outline">Music</li>
                                    <li class="tag-default tag-pill tag-outline">Song</li>
                                </ul>
                            </a>
                        </div>

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
    view! {
        <div class="article-page">
            <div class="banner">
                <div class="container">
                    <h1>How to build webapps that scale</h1>

                    <div class="article-meta">
                        <a href="/profile/eric-simons">
                            <img src="http://i.imgur.com/Qr71crq.jpg"/>
                        </a>
                        <div class="info">
                            <a href="/profile/eric-simons" class="author">
                                Eric Simons
                            </a>
                            <span class="date">January 20th</span>
                        </div>
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
                    </div>
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
                    <div class="article-meta">
                        <a href="profile.html">
                            <img src="http://i.imgur.com/Qr71crq.jpg"/>
                        </a>
                        <div class="info">
                            <a href="" class="author">
                                Eric Simons
                            </a>
                            <span class="date">January 20th</span>
                        </div>

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
                    </div>
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
