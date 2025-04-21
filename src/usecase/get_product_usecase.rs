use anyhow::Result;

use crate::infrastructure::persistence::repositories::product_repository::ProductRepository;

use super::command::get_product_command::GetProductCommand;

pub struct GetProductUseCase {
    product_repository: ProductRepository,
}

impl GetProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<GetProductCommand> {
        print!("->> get_product_usecase");
        let product = self.product_repository.find_by_id(id).await?;
        Ok(product.into())
    }
}
