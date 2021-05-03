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
}

///
/// Run the application as a server
///
pub async fn run(pg_pool: sqlx::PgPool) {
    server::serve(pg_pool).await;
}
