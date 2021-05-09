pub mod config;

mod bus;
mod error;
mod repository;
mod server;

pub mod schema {
    pub mod event;
    pub mod mutation;
    pub mod query;
    pub mod subscription;
    pub mod todo_item;

    pub type AppSchema =
        async_graphql::Schema<query::Query, mutation::Mutation, subscription::Subscription>;
}

///
/// Run the application as a server
///
pub async fn run(port: Option<u16>, pg_pool: sqlx::PgPool) {
    server::serve(port, pg_pool).await;
}
