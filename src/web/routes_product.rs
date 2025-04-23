use axum::extract::Path;
use axum::{routing::post, routing::get, Json, Router};

use crate::error::{Error, Result};
use crate::usecase::buy_product_usecase::BuyProductUseCase;
use crate::usecase::commands::buy_product_command::BuyProductCommand;
use crate::usecase::queries::get_product_query::GetProductQuery;
use crate::usecase::get_all_products_usecase::GetAllProductsUseCase;
use crate::usecase::get_product_usecase::GetProductUseCase;

pub fn routes() -> Router {
    Router::new()
    .route("/products", get(get_all_products))
    .route("/products/{id}", get(get_product))
    .route("/products/{id}/buy", post(buy_product))
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
async fn get_all_products() -> Result<Json<Vec<GetProductQuery>>> {
    let get_all_products_usecase = GetAllProductsUseCase::new();
    let products = get_all_products_usecase.get_all().await?;
    Ok(Json(products))
}

async fn get_product(Path(id): Path<u32>) -> Result<Json<GetProductQuery>> {
    let get_product_usecase = GetProductUseCase::new();
    let product = get_product_usecase.get_by_id(id).await?;
    Ok(Json(product))
}