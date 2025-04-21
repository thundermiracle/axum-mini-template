use axum::{routing::post, routing::get, Json, Router};
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::usecase::buy_product_usecase::BuyProductUseCase;
use crate::usecase::command::get_product_command::GetProductCommand;
use crate::usecase::get_all_products_usecase::GetAllProductsUseCase;

#[derive(Debug, Deserialize)]
pub struct BuyProductParams {
    pub id: u32,
    pub quantity: u32,
}

pub fn routes() -> Router {
    Router::new()
        .route("/products/buy", post(buy_product))
        .route("/products", get(get_all_products))
}

/**
 * POST /products/buy
 */
async fn buy_product(Json(params): Json<BuyProductParams>) -> Result<()> {
    let buy_product_usecase = BuyProductUseCase::new();
    buy_product_usecase.buy(params.id, params.quantity).await.map_err(|_| Error::BuyProductFailed)
}

/**
 * GET /products
 */
async fn get_all_products() -> Result<Json<Vec<GetProductCommand>>> {
    let get_all_products_usecase = GetAllProductsUseCase::new();
    let products = get_all_products_usecase.get_all().await?;
    Ok(Json(products))
}
