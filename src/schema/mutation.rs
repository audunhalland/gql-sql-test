use crate::repository::Repository;
use crate::{bus::EventBus, model::AppError};

use super::{event::Event, todo_item::TodoItem};

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

    async fn create_item(
        &self,
        ctx: &async_graphql::Context<'_>,
        description: String,
    ) -> Result<TodoItem, AppError> {
        let repository = ctx.data_unchecked::<Repository>();
        let item = repository.insert_todo_item(&description).await?;

        Ok(item)
    }

    async fn set_done(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        let repository = ctx.data_unchecked::<Repository>();
        let success = repository.set_done(id).await?;

        Ok(success)
    }
}
