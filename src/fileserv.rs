use crate::app::App;
use axum::response::Response;
use axum::{body::Body, extract::State, response::IntoResponse};
use leptos::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: http::Uri,
    State(options): State<LeptosOptions>,
    req: http::Request<Body>,
) -> Response {
    let res = get_static_file(uri, &options.site_root).await;

    if res.status() == http::StatusCode::OK {
        res
    } else {
        let handler = leptos_axum::render_app_to_stream(options, App);
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: http::Uri, root: &str) -> Response {
    let req = http::Request::builder()
        .uri(uri.clone())
        .body(())
        .expect("uri can't fail");
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    let res = ServeDir::new(root).oneshot(req).await.expect("infallible");
    res.into_response()
}
