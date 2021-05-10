use sqlx::Connection;

use gql_sql_test::repository::Repository;

#[tokio::test]
async fn should_get_empty_todo_list_on_empty_database() {
    let todo_items = Repository::new(create_test_db().await)
        .fetch_todo_items(None, 0..10)
        .await
        .unwrap();

    assert_eq!(todo_items, vec![]);
}

#[tokio::test]
async fn should_insert_a_new_todo_item_and_then_fetch_it() {
    let repository = Repository::new(create_test_db().await);

    let inserted_todo_item = repository.insert_todo_item("foobar").await.unwrap();
    let todo_items = repository.fetch_todo_items(None, 0..10).await.unwrap();

    assert_eq!(todo_items, vec![inserted_todo_item]);
}

///
/// Create a "fresh" Postgres database for running tests with.
///
/// In Rust, test run in parallel. To keep them isolated, they should use different
/// databases. The solution for now is to create one database per test thread,
/// and derive the database name from the thread name, delete the database
/// if it already exists, (re)create it and at last run migrations.
///
async fn create_test_db() -> sqlx::PgPool {
    let database_name = format!(
        "test_db_{}",
        std::thread::current().name().unwrap().to_string()
    );
    let url_without_database = "postgres://rust:rust@localhost:9876";

    let mut connection = sqlx::PgConnection::connect(url_without_database)
        .await
        .unwrap();

    sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{}""#, database_name))
        .execute(&mut connection)
        .await
        .expect("failed to drop");

    // Create a new database
    sqlx::query(&format!(r#"CREATE DATABASE "{}""#, database_name))
        .execute(&mut connection)
        .await
        .expect("failed creating test database");

    let pg_pool = sqlx::PgPool::connect(&format!("{}/{}", url_without_database, database_name))
        .await
        .expect("Failed to connect to database");

    // Execute our migration files
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");

    pg_pool
}
