use axum::extract::Path;
use axum::{routing::post, routing::get, Json, Router};

use crate::error::{Error, Result};
use crate::usecase::buy_product_usecase::BuyProductUseCase;
use crate::usecase::command::buy_product_command::BuyProductCommand;
use crate::usecase::command::get_product_command::GetProductCommand;
use crate::usecase::get_all_products_usecase::GetAllProductsUseCase;

pub fn routes() -> Router {
    Router::new()
        .route("/products/{id}/buy", post(buy_product))
        .route("/products", get(get_all_products))
}

/**
 * POST /products/buy
 */
async fn buy_product(Path(id): Path<u32>, Json(command): Json<BuyProductCommand>) -> Result<()> {
    let buy_product_usecase = BuyProductUseCase::new();
    buy_product_usecase.buy(id, command).await.map_err(|_| Error::BuyProductFailed)
}

/**
 * GET /products
 */
async fn get_all_products() -> Result<Json<Vec<GetProductCommand>>> {
    let get_all_products_usecase = GetAllProductsUseCase::new();
    let products = get_all_products_usecase.get_all().await?;
    Ok(Json(products))
}
