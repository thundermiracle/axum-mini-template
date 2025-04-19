use anyhow::Result;

use crate::infrastructure::product_repository::ProductRepository;

pub struct BuyProductUseCase {
    product_repository: ProductRepository,
}

impl BuyProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }

    pub async fn buy(&self, product_id: u32, amount: u32) -> Result<()> {
        let mut product = self.product_repository.find_by_id(product_id).await?;
        product.sell(amount)?;
        self.product_repository.save(product).await?;
        Ok(())
    }
}
