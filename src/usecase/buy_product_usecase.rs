use anyhow::Result;

use crate::infrastructure::persistence::repositories::product_repository::ProductRepository;

use super::command::buy_product_command::BuyProductCommand;

pub struct BuyProductUseCase {
    product_repository: ProductRepository,
}

impl BuyProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
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
