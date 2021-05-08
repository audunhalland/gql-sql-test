use gql_sql_test::config::Config;

#[tokio::main]
async fn main() {
    let config = Config {
        db_url: "postgres://rust:rust@localhost:9876/rust".to_string(),
    };
    let pg_pool = sqlx::PgPool::connect(&config.db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    gql_sql_test::run(Some(8000), pg_pool).await;
}
