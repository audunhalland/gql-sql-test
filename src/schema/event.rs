#[derive(async_graphql::SimpleObject, Clone)]
pub struct Event {
    pub id: uuid::Uuid,
}
