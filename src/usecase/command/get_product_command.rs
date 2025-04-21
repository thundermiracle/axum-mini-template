use crate::domain::product::Product;
use serde::Serialize;

#[derive(Serialize)]
pub struct GetProductCommand {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl Into<GetProductCommand> for Product {
    fn into(self) -> GetProductCommand {
        GetProductCommand {
            id: self.id,
            name: self.name,
            price: self.price,
            description: self.description,
            quantity: self.quantity,
        }
    }
}