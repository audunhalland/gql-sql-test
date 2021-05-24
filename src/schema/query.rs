use crate::error::AppError;
use crate::model::TodoFilter;
use crate::repository::Repository;

use super::todo_item::TodoItem;

///
/// The root of the GraphQL 'Query' type
///
pub struct Query;

#[async_graphql::Object]
impl Query {
    ///
    /// Query our todo items.
    ///
    async fn todo_items(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<TodoItem>, AppError> {
        let repository = ctx.data_unchecked::<Repository>();
        let todo_items: Vec<TodoItem> = repository
            .list_todo_items(TodoFilter {
                ids: None,
                range: 0..20,
            })
            .await?;

        Ok(todo_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{value, EmptyMutation, EmptySubscription};

    #[tokio::test]
    async fn query_todos_should_work() {
        let mut mock_repo = Repository::faux();
        faux::when!(mock_repo.list_todo_items(_)).then_return(Ok(vec![TodoItem {
            id: uuid::Uuid::nil(),
            description: "test".to_string(),
            done: false,
        }]));

        let response = async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(mock_repo)
            .finish()
            .execute("{ todoItems { id, description } }")
            .await;

        assert_eq!(
            response.data,
            value!({
                "todoItems": [{
                    "id": uuid::Uuid::nil().to_string(),
                    "description": "test"
                }]
            })
        );
    }
}
