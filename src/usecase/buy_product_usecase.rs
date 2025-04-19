use anyhow::Result;

use crate::infrastructure::product_repository::ProductRepository;

pub struct BuyProductUseCase {}

impl BuyProductUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn buy(&self, product_id: u32, amount: u32) -> Result<()> {
        let mut product = ProductRepository::find_by_id(product_id).await?;
        product.sell(amount)?;
        ProductRepository::save(product).await?;
        Ok(())
    }
}
