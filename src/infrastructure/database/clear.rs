use anyhow::Result;
use sqlx::Executor;

use crate::infrastructure::database::db::get_db;

pub async fn clear_database() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    
    // サンプルデータを挿入
    pool.execute(
        r#"
        DELETE FROM products;
        DELETE FROM sqlite_sequence WHERE name = 'products';
        "#
    ).await?;
    
    Ok(())
}
