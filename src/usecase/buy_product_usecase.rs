use anyhow::Result;

use crate::domain::product::Product;

pub struct BuyProductUseCase {}

impl BuyProductUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn buy(&self, product_id: u32, amount: u32) -> Result<()> {
        todo!()
    }
}
