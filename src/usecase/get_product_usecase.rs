use anyhow::Result;

use crate::infrastructure::persistence::SqliteProductRepository;

use super::queries::GetProductQuery;

pub struct GetProductUseCase {
    product_repository: SqliteProductRepository,
}

impl GetProductUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: SqliteProductRepository::new(),
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<GetProductQuery> {
        print!("->> get_product_usecase");
        let product = self.product_repository.find_by_id(id).await?;
        Ok(product.into())
    }
}
