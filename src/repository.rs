use crate::schema::todo_item::TodoItem;

pub struct Repository {
    pool: sqlx::PgPool,
}

impl Repository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_todo_items(&self) -> Result<Vec<TodoItem>, sqlx::Error> {
        sqlx::query_as!(TodoItem, "SELECT id from todo_item")
            .fetch_all(&self.pool)
            .await
    }
}
