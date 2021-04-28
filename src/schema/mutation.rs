///
/// The root of the GraphQL 'Query' type
///
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn send_event(&self, _ctx: &async_graphql::Context<'_>) -> bool {
        // TODO: implement
        false
    }
}
