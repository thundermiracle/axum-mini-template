use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Executor, Sqlite};

use crate::infrastructure::database::db::get_db;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    // データベースが存在しない場合は作成
    if !Sqlite::database_exists(database_url).await? {
        Sqlite::create_database(database_url).await?;
    }

    // テーブル作成
    let db = get_db().await?;
    let pool = db.get_pool();

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            price INTEGER NOT NULL,
            description TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .await?;

    Ok(())
} 