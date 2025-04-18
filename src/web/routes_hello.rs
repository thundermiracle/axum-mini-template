use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;

// #region ----------- /routes_hello -----------
pub fn routes() -> Router {
    Router::new()
        .route("/hello-query", get(hello_query_handler))
        .route("/hello-path/{first_name}/{last_name}", get(hello_path_handler))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_query_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("<<- in hello_query_handler: {:?}", params);

    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("<h1>Hello query version: {}!</h1>", name))
}

async fn hello_path_handler(
    Path((first_name, last_name)): Path<(String, String)>,
) -> impl IntoResponse {
    println!("<<- in hello_path_handler: {first_name}, {last_name}");

    Html(format!("<h2>Hello path version: {} {}!!</h2>", last_name, first_name))
}
