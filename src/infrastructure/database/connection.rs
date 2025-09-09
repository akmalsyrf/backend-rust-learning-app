use anyhow::Result;
use sqlx::PgPool;
use std::env;

pub struct DatabaseConnection {
    pub pool: PgPool,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:password@localhost:5432/rust_learning".to_string()
        });

        let pool = PgPool::connect(&database_url).await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn health_check(&self) -> Result<()> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query("SELECT 1").fetch_one(&mut *conn).await?;
        Ok(())
    }
}
