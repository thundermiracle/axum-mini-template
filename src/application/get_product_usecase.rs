use anyhow::Result;
use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use super::queries::GetProductQuery;

pub struct GetProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<GetProductQuery> {
        print!("->> get_product_usecase");
        let product = self.product_repository.find_by_id(id).await?;
        Ok(product.into())
    }
}
