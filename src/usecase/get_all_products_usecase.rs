use anyhow::Result;

use crate::infrastructure::persistence::SqliteProductRepository;

use super::queries::GetProductQuery;

pub struct GetAllProductsUseCase {
    product_repository: SqliteProductRepository,
}

impl GetAllProductsUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: SqliteProductRepository::new(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<GetProductQuery>> {
        print!("->> get_all_products_usecase");
        let products = self.product_repository.find_all().await?;
        Ok(products.into_iter().map(|p| p.into()).collect())
    }
}