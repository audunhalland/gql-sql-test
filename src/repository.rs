use std::ops::Range;

use crate::error::AppError;
use crate::schema::todo_item::TodoItem;

///
/// A mockable Repository abstraction.
/// The `faux::create` macro generates `::faux()` static method.
/// It returns a mockable version of the struct.
///
/// `cfg_attr` is a way of doing conditional compilation. `test` means
/// that this is gated on being a _test build_ (avoid paying this cost in production)
///
/// `faux` works by actually modifying the type, and when it's active
/// it modifies the type so that it may be one of two variants at runtime:
/// A mocked instance instatiated by `::faux()` or a real instance
/// instatnated by `::new()`.
///
#[cfg_attr(test, faux::create)]
pub struct Repository {
    pool: sqlx::PgPool,
}

#[cfg_attr(test, faux::methods)]
impl Repository {
    /// Construct a new Repository.
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Fetch todo items.
    pub async fn fetch_todo_items(
        &self,
        ids: Option<&[uuid::Uuid]>,
        range: Range<usize>,
    ) -> Result<Vec<TodoItem>, AppError> {
        let rows = sqlx::query_as!(
            TodoItem,
            "SELECT id, description, done
            FROM todo_item
            WHERE
                id = any($1) OR $1 IS NULL
            OFFSET $2
            LIMIT $3",
            ids,
            range.start as u32,
            range.end as u32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
