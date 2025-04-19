use axum::{routing::post, Json, Router};
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::usecase::buy_product_usecase::BuyProductUseCase;

#[derive(Debug, Deserialize)]
pub struct BuyProductParams {
    pub id: u32,
    pub amount: u32,
}

pub fn routes() -> Router {
    Router::new()
        .route("/products/buy", post(buy_product_handler))
}

/**
 * POST /products/buy
 */
async fn buy_product_handler(Json(params): Json<BuyProductParams>) -> Result<()> {
    let buy_product_usecase = BuyProductUseCase::new();
    buy_product_usecase.buy(params.id, params.amount).await.map_err(|_| Error::BuyProductFailed)
}
