use crate::error::AppError;
use crate::repository::Repository;

use super::todo_item::TodoItem;

///
/// The root of the GraphQL 'Query' type
///
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn todo_items(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<TodoItem>, AppError> {
        let todo_items = ctx
            .data_unchecked::<Repository>()
            .fetch_todo_items(0..20)
            .await?;

        Ok(todo_items)
    }
}
