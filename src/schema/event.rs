#[derive(async_graphql::SimpleObject)]
pub struct Event {
    pub id: uuid::Uuid,
}
