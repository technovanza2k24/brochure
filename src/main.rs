use axum::{response::Html, routing::get};
use std::io::{BufReader, Read};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    let css_dir = ServeDir::new("./static/css/");
    let js_dir = ServeDir::new("./static/js/");
    let template_dir = ServeDir::new("./static/html/");
    let router = axum::Router::new()
        .route("/", get(index))
        .nest_service("/static/html/", template_dir)
        .nest_service("/static/css/", css_dir)
        .nest_service("/static/js/", js_dir);
    axum::serve(listener, router).await.unwrap();
}

async fn index() -> axum::response::Result<Html<String>> {
    let handle = std::fs::File::open("./static/html/index.html").unwrap();
    let mut buffered = BufReader::new(handle);
    let mut buf = String::new();
    buffered.read_to_string(&mut buf).unwrap();
    Ok(Html(buf))
}
