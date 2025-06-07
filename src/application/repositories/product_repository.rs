use anyhow::Result;
use crate::domain::models::Product;

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>>;
    async fn find_by_id(&self, id: u32) -> Result<Product>;
    async fn save(&self, product: Product) -> Result<()>;
}
