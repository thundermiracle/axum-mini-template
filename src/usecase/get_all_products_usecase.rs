use anyhow::Result;

use crate::infrastructure::persistence::repositories::product_repository::ProductRepository;

use super::queries::get_product_query::GetProductQuery;

pub struct GetAllProductsUseCase {
    product_repository: ProductRepository,
}

impl GetAllProductsUseCase {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<GetProductQuery>> {
        print!("->> get_all_products_usecase");
        let products = self.product_repository.find_all().await?;
        Ok(products.into_iter().map(|p| p.into()).collect())
    }
}