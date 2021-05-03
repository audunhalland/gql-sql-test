use super::todo_item::TodoItem;

///
/// The root of the GraphQL 'Query' type
///
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn todo_items(&self) -> Option<Vec<TodoItem>> {
        None
    }
}
