use std::ops::Range;

use crate::error::AppError;
use crate::schema::todo_item::TodoItem;

pub struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_todo_items(&self, range: Range<usize>) -> Result<Vec<TodoItem>, AppError> {
        let rows = sqlx::query_as!(
            TodoItem,
            "SELECT id, description, done FROM todo_item OFFSET $1 LIMIT $2",
            range.start as u32,
            range.end as u32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
