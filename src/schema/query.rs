
use super::data::Data;

///
/// The root of the GraphQL 'Query' type
///
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn stuff(&self) -> Option<Data> {
        None
    }
}
