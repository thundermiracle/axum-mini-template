use anyhow::{anyhow, Result};

pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl Product {
    pub fn new(id: u32, name: String, price: u32, description: String, quantity: u32) -> Self {
        Self {
            id,
            name,
            price,
            description,
            quantity,
        }
    }

    pub fn sell(&mut self, quantity: u32) -> Result<()> {
        if quantity > self.quantity {
            return Err(anyhow!("Not enough quantity"));
        }
        self.quantity -= quantity;

        Ok(())
    }
}