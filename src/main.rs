use axum::{
    Router,
    middleware,
    response::Response,
};
pub use error::{Error, Result};

mod error;
mod web;
mod usecase;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_product::routes())
        .layer(middleware::map_response(main_response_mapper));

    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("->> Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> main response mapper");
    println!();

    res
}
