#[derive(async_graphql::SimpleObject)]
pub struct TodoItem {
    pub id: uuid::Uuid,
}
