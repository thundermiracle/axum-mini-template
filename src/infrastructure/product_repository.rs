use anyhow::{anyhow, Result};
use sqlx::Row;
use chrono::Utc;

use crate::domain::product::Product;
use crate::infrastructure::db::get_db;
use crate::infrastructure::product_entity::ProductEntity;

pub struct ProductRepository;

impl ProductRepository {
    pub fn new() -> Self {
        Self {}
    }
    
    // エンティティからドメインモデルへのマッピング
    fn entity_to_domain(entity: ProductEntity) -> Product {
        Product::new(
            entity.id,
            entity.name,
            entity.price,
            entity.description,
            entity.amount,
        )
    }
}

impl ProductRepository {
    pub async fn find_all(&self) -> Result<Vec<Product>> {
        let db = get_db().await?;
        let pool = db.get_pool();
        
        let rows = sqlx::query("SELECT * FROM products")
            .fetch_all(pool)
            .await?;
        
        let products = rows
            .iter()
            .map(|row| {
                let entity = ProductEntity {
                    id: row.get("id"),
                    name: row.get("name"),
                    price: row.get("price"),
                    description: row.get("description"),
                    amount: row.get("amount"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                
                Self::entity_to_domain(entity)
            })
            .collect::<Vec<Product>>();
        
        Ok(products)
    }

    pub async fn find_by_id(&self, id: u32) -> Result<Product> {
        let db = get_db().await?;
        let pool = db.get_pool();
        
        let row = sqlx::query("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        
        match row {
            Some(row) => {
                let entity = ProductEntity {
                    id: row.get("id"),
                    name: row.get("name"),
                    price: row.get("price"),
                    description: row.get("description"),
                    amount: row.get("amount"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                
                Ok(Self::entity_to_domain(entity))
            },
            None => Err(anyhow!("Product not found: {}", id)),
        }
    }

    pub async fn save(&self, product: Product) -> Result<Product> {
        let db = get_db().await?;
        let pool = db.get_pool();
        
        let now = Utc::now().to_rfc3339();
        
        // 既存のプロダクトを検索
        let existing = sqlx::query("SELECT * FROM products WHERE id = ?")
            .bind(product.id)
            .fetch_optional(pool)
            .await?;
        
        match existing {
            // 更新
            Some(_) => {
                sqlx::query(
                    "UPDATE products SET name = ?, price = ?, description = ?, amount = ?, updated_at = ? WHERE id = ?"
                )
                .bind(&product.name)
                .bind(product.price)
                .bind(&product.description)
                .bind(product.amount)
                .bind(&now)
                .bind(product.id)
                .execute(pool)
                .await?;
            },
            // 新規作成
            None => {
                sqlx::query(
                    "INSERT INTO products (name, price, description, amount, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&product.name)
                .bind(product.price)
                .bind(&product.description)
                .bind(product.amount)
                .bind(&now)
                .bind(&now)
                .execute(pool)
                .await?;
            }
        }
        
        Ok(product)
    }
}
