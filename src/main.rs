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
async fn main() -> anyhow::Result<()> {
    // SQLiteデータベースの初期化
    let database_url = "sqlite:db.sqlite";
    infrastructure::db::init_db(database_url).await?;
    infrastructure::migrations::run_migrations(database_url).await?;
    
    // 開発環境でのみシーディングを実行
    #[cfg(debug_assertions)]
    infrastructure::seed::seed_database().await?;

    let app = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_product::routes())
        .layer(middleware::map_response(main_response_mapper));

    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("->> Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> main response mapper");
    println!();

    res
}
