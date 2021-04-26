struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn test(&self) -> Result<(), sqlx::Error> {
        let lol: Vec<_> = sqlx::query!("SELECT id from lol")
            .fetch_all(&self.pool)
            .await?;

        Ok(())
    }
}
