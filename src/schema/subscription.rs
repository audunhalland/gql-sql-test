use futures::Stream;

use super::event::Event;

///
/// The root of the GraphQL 'Subscription' type
///
pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn events(&self) -> impl Stream<Item = Event> {
        futures::stream::empty()
    }
}
