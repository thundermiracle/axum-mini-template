use anyhow::Result;
use std::sync::Arc;

use crate::usecase::repositories::ProductRepository;
use super::commands::BuyProductCommand;

pub struct BuyProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl BuyProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
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
