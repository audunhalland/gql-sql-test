use futures::Stream;
use futures::TryStreamExt;

use crate::bus::EventBus;

use super::event::Event;

///
/// The root of the GraphQL 'Subscription' type
///
pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn events(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> impl Stream<Item = Result<Event, async_graphql::Error>> {
        let receiver = ctx.data_unchecked::<EventBus>().subscribe();

        tokio_stream::wrappers::BroadcastStream::new(receiver).map_err(|err| {
            async_graphql::Error::new(format!("Failed to generate next event: {:?}", err))
        })
    }
}
