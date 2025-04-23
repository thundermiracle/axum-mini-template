use anyhow::Result;

use crate::infrastructure::persistence::SqliteProductRepository;

use super::commands::BuyProductCommand;

pub struct BuyProductUseCase {
    product_repository: SqliteProductRepository,
}

impl BuyProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: SqliteProductRepository::new(),
        }
    }

    pub async fn buy(&self, product_id: u32, command: BuyProductCommand) -> Result<()> {
        print!("->> buy_product_usecase");
        let mut product = self.product_repository.find_by_id(product_id).await?;
        product.sell(command.quantity)?;
        self.product_repository.save(product).await?;
        Ok(())
    }
}
