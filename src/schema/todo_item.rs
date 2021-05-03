#[derive(async_graphql::SimpleObject, Clone)]
pub struct TodoItem {
    pub id: uuid::Uuid,
    pub description: String,
    pub done: bool,
}
