use axum::extract::Path;
use axum::extract::State;
use axum::{routing::post, routing::get, Json, Router};
use std::sync::Arc;

use crate::DI::Container;
use crate::error::{Error, Result};
use crate::usecase::commands::BuyProductCommand;
use crate::usecase::queries::GetProductQuery;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
    .route("/products", get(get_all_products))
    .route("/products/{id}", get(get_product))
    .route("/products/{id}/buy", post(buy_product))
}

/**
 * POST /products/buy
 */
async fn buy_product(
    State(container): State<Arc<Container>>,
    Path(id): Path<u32>, 
    Json(command): Json<BuyProductCommand>
) -> Result<()> {
    let buy_product_usecase = container.create_buy_product_usecase();
    buy_product_usecase.buy(id, command).await.map_err(|_| Error::BuyProductFailed)
}

/**
 * GET /products
 */
async fn get_all_products(
    State(container): State<Arc<Container>>
) -> Result<Json<Vec<GetProductQuery>>> {
    let get_all_products_usecase = container.create_get_all_products_usecase();
    let products = get_all_products_usecase.get_all().await?;
    Ok(Json(products))
}

async fn get_product(
    State(container): State<Arc<Container>>,
    Path(id): Path<u32>
) -> Result<Json<GetProductQuery>> {
    let get_product_usecase = container.create_get_product_usecase();
    let product = get_product_usecase.get_by_id(id).await?;
    Ok(Json(product))
}