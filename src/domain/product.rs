use anyhow::{anyhow, Result};

pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub amount: u32,
}

impl Product {
    pub fn new(id: u32, name: String, price: u32, description: String, amount: u32) -> Self {
        Self {
            id,
            name,
            price,
            description,
            amount,
        }
    }

    pub fn sell(&mut self, amount: u32) -> Result<()> {
        if amount > self.amount {
            return Err(anyhow!("Not enough amount"));
        }
        self.amount -= amount;

        Ok(())
    }
}