use anyhow::Result;
use chrono::Utc;
use sqlx::Executor;

use crate::infrastructure::database::db::get_db;

pub async fn seed_database() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    
    let now = Utc::now().to_rfc3339();
    
    // サンプルデータを挿入
    pool.execute(
        format!(
            r#"
            INSERT INTO products (name, price, description, amount, created_at, updated_at)
            VALUES 
            ('ノートPC', 150000, '高性能ノートパソコン', 10, '{}', '{}'),
            ('スマートフォン', 80000, '最新型スマートフォン', 20, '{}', '{}'),
            ('ヘッドフォン', 25000, 'ノイズキャンセリングヘッドフォン', 30, '{}', '{}')
            "#,
            now, now, now, now, now, now
        ).as_str()
    )
    .await?;
    
    Ok(())
} 