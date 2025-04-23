use anyhow::Result;

use crate::infrastructure::persistence::repositories::product_repository::ProductRepository;

use super::query::get_product_query::GetProductQuery;

pub struct GetProductUseCase {
    product_repository: ProductRepository,
}

impl GetProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<GetProductQuery> {
        print!("->> get_product_usecase");
        let product = self.product_repository.find_by_id(id).await?;
        Ok(product.into())
    }
}
