pub mod config;

mod bus;
mod repository;
mod server;

pub mod schema {
    pub mod data;
    pub mod event;
    pub mod query;
    pub mod subscription;
}

///
/// Run the application as a server
///
pub async fn run(pg_pool: sqlx::PgPool) {
    server::serve(pg_pool).await;
}
