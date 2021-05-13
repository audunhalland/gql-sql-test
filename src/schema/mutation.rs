use crate::bus::EventBus;
use crate::repository::Repository;

use super::event::Event;

///
/// The root of the GraphQL 'Query' type
///
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn send_event(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<usize> {
        let event = Event {
            id: uuid::Uuid::new_v4(),
        };

        ctx.data_unchecked::<EventBus>()
            .sender()
            .send(event)
            .map_err(|err| async_graphql::Error::new(format!("failed to send: {:?}", err)))
    }

    async fn set_done(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: uuid::Uuid,
    ) -> async_graphql::Result<bool> {
        let repository = ctx.data_unchecked::<Repository>();
        let success = repository.set_done(id).await?;

        Ok(success)
    }
}
