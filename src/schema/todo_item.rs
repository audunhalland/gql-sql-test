#[derive(async_graphql::SimpleObject)]
pub struct TodoItem {
    pub id: uuid::Uuid,
    pub description: String,
    pub done: bool,
}
