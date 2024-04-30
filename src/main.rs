#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{extract::Path, routing::get, Router};
    use demo_app::app::App;
    use demo_app::auth;
    use demo_app::fileserv::file_and_error_handler;
    use leptos::{get_configuration, logging};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::{compression::CompressionLayer, trace::TraceLayer};
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    demo_app::db::init().await;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    async fn get_raw_md(
        Path((author, slug)): Path<(String, String)>,
    ) -> Result<String, http::StatusCode> {
        sqlx::query_scalar!(
            "select body from article where author = ? and slug = ?",
            author,
            slug
        )
        .fetch_one(demo_app::db::get())
        .await
        .map_err(|_| http::StatusCode::NOT_FOUND)
    }

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .route("/raw/article/:author/:slug", get(get_raw_md))
        .fallback(file_and_error_handler)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(auth::server::auth_middleware))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
