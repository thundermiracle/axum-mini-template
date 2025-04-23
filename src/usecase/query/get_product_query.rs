use crate::domain::models::product::Product;
use serde::Serialize;

#[derive(Serialize)]
pub struct GetProductQuery {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl Into<GetProductQuery> for Product {
    fn into(self) -> GetProductQuery {
        GetProductQuery {
            id: self.id,
            name: self.name,
            price: self.price,
            description: self.description,
            quantity: self.quantity,
        }
    }
}