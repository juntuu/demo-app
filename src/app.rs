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
                </Routes>
                <Footer/>
            </main>
        </Router>
    }
}

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
                                New Article
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/settings">
                                <i class="ion-gear-a"></i>
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
