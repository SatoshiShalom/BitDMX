/// Storage layer for persistent state
use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn initialize(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS batches (
                id INTEGER PRIMARY KEY,
                state_root BLOB NOT NULL,
                timestamp INTEGER NOT NULL,
                transaction_count INTEGER NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS proofs (
                batch_id INTEGER PRIMARY KEY,
                proof_data BLOB NOT NULL,
                commitment BLOB NOT NULL,
                FOREIGN KEY (batch_id) REFERENCES batches(id)
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_batch(&self, batch: &crate::batch::Batch) -> Result<()> {
        sqlx::query(
            "INSERT INTO batches (id, state_root, timestamp, transaction_count) VALUES (?, ?, ?, ?)"
        )
        .bind(batch.id as i64)
        .bind(&batch.state_root[..])
        .bind(batch.timestamp as i64)
        .bind(batch.transactions.len() as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
