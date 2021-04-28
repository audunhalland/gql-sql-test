#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct Event {
    pub id: uuid::Uuid,
}
