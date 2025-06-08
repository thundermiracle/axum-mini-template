use axum::extract::Path;
use axum::extract::State;
use axum::{routing::post, routing::get, Json, Router};
use std::sync::Arc;

use crate::DI::Container;
use crate::error::{Error, Result};
use crate::application::commands::BuyProductCommand;
use crate::application::ApplicationError;
use crate::presentation::{ProductDto, BuyProductDto};

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
    .route("/products", get(get_all_products))
    .route("/products/{id}", get(get_product))
    .route("/products/{id}/buy", post(buy_product))
}

/**
 * POST /products/{id}/buy
 */
async fn buy_product(
    State(container): State<Arc<Container>>,
    Path(id): Path<u32>, 
    Json(dto): Json<BuyProductDto>
) -> Result<()> {
    let buy_product_usecase = container.create_buy_product_usecase();
    
    // DTOからコマンドへの変換
    let command = BuyProductCommand {
        quantity: dto.quantity,
    };
    
    buy_product_usecase
        .buy(id, command)
        .await
        .map_err(|e| match e {
            ApplicationError::ProductNotFound(_) => Error::NotFound,
            ApplicationError::Domain(_) => Error::BuyProductFailed,
            _ => Error::InternalServerError,
        })
}

/**
 * GET /products
 */
async fn get_all_products(
    State(container): State<Arc<Container>>
) -> Result<Json<Vec<ProductDto>>> {
    let get_all_products_usecase = container.create_get_all_products_usecase();
    
    let products = get_all_products_usecase
        .get_all()
        .await
        .map_err(|_| Error::InternalServerError)?;
        
    let dtos: Vec<ProductDto> = products.into_iter().map(|p| p.into()).collect();
    Ok(Json(dtos))
}

/**
 * GET /products/{id}
 */
async fn get_product(
    State(container): State<Arc<Container>>,
    Path(id): Path<u32>
) -> Result<Json<ProductDto>> {
    let get_product_usecase = container.create_get_product_usecase();
    
    let product = get_product_usecase
        .get_by_id(id)
        .await
        .map_err(|e| match e {
            ApplicationError::ProductNotFound(_) => Error::NotFound,
            _ => Error::InternalServerError,
        })?;
        
    Ok(Json(product.into()))
}