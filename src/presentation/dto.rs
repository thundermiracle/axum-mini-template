use serde::{Deserialize, Serialize};
use crate::application::queries::GetProductQuery;

/// 商品のHTTPレスポンス用DTO
#[derive(Serialize, Deserialize)]
pub struct ProductDto {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

/// 商品購入コマンドのHTTPリクエスト用DTO
#[derive(Serialize, Deserialize)]
pub struct BuyProductDto {
    pub quantity: u32,
}

// Application層のQueryからPresentation層のDTOへの変換
impl From<GetProductQuery> for ProductDto {
    fn from(query: GetProductQuery) -> Self {
        ProductDto {
            id: query.id,
            name: query.name,
            price: query.price,
            description: query.description,
            quantity: query.quantity,
        }
    }
} 