use anyhow::Result;

use crate::infrastructure::product_repository::ProductRepository;

use super::command::get_product_command::GetProductCommand;

pub struct GetAllProductsUseCase {
    product_repository: ProductRepository,
}

impl GetAllProductsUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<GetProductCommand>> {
        let products = self.product_repository.find_all().await?;
        Ok(products.into_iter().map(|p| p.into()).collect())
    }
}