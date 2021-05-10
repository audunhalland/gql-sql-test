use gql_sql_test::config::Config;

#[tokio::main]
async fn main() {
    let config = Config {
        db_url: "postgres://rust:rust@localhost:9876/rust".to_string(),
    };
    let pg_pool = sqlx::PgPool::connect(&config.db_url)
        .await
        .expect("Failed to connect to database");

    // The `migrate!` macro _embeds_ the migration files into the resulting binary,
    // so there is no need to worry about the filesystem during runtime
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    gql_sql_test::run(Some(8000), pg_pool).await;
}
