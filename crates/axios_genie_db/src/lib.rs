mod challange;

use sqlx::SqlitePool;
use anyhow::Result;

#[derive(Debug)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;
        Ok(Self { pool })
    }
}

impl From<SqlitePool> for Db {
    fn from(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
