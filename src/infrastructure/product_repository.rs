use anyhow::{anyhow, Result};

use crate::domain::product::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn find_all() -> Result<Vec<Product>> {
        todo!()
    }

    pub async fn find_by_id(id: u32) -> Result<Product> {
        todo!()
    }

    pub async fn save(product: Product) -> Result<Product> {
        todo!()
    }
}