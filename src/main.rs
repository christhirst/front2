use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use dioxus_sortable::Sortable;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        Router::new()
            .route("/", get(app_endpoint))
            .into_make_service(),
    )
    .await
    .unwrap();
}
async fn app_endpoint() -> Html<String> {
    // render the rsx! macro to HTML
    Html(dioxus_ssr::render_element(rsx! {
        th {}
    }))
}
