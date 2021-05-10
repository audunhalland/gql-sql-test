use sqlx::Connection;

use gql_sql_test::repository::Repository;

#[tokio::test]
async fn should_get_empty_todo_list_on_empty_database() {
    let test_db = TestDb::new().await;

    let todo_items = Repository::new(test_db.pg_pool.clone())
        .fetch_todo_items(None, 0..0)
        .await
        .unwrap();

    assert_eq!(todo_items, vec![]);
}

///
/// A fresh database used for tests.
/// This database is _deleted_ when the struct goes out of scope. See the `Drop` impl below.
///
struct TestDb {
    pg_pool: sqlx::PgPool,
    database_name: uuid::Uuid,
}

impl TestDb {
    async fn new() -> Self {
        let database_name = uuid::Uuid::new_v4();
        let url_without_database = "postgres://rust:rust@localhost:9876";
        let mut connection = sqlx::PgConnection::connect(url_without_database)
            .await
            .unwrap();

        // Create a new database
        sqlx::query(&format!("CREATE DATABASE \"{}\"", database_name))
            .execute(&mut connection)
            .await
            .unwrap();

        let pg_pool = sqlx::PgPool::connect(&format!("{}/{}", url_without_database, database_name))
            .await
            .expect("Failed to connect to database");

        // Execute our migration files
        sqlx::migrate!()
            .run(&pg_pool)
            .await
            .expect("Failed to migrate");

        Self {
            pg_pool,
            database_name,
        }
    }
}

// The Drop trait is how to implement a destructor in Rust.
impl Drop for TestDb {
    fn drop(&mut self) {
        let pg_pool = self.pg_pool.clone();
        let database_name = self.database_name;

        // Note that `drop` is not an async function. Rust does not support async destructors (yet, at least).
        // What we can instead do is to _spawn_ a future onto the locally running executor (on the current thread).
        //
        // So, what happens when an asynchronous test function returns? TestDb gets dropped,
        // then this future gets spawned. The `tokio::test` macro is generating code for running the
        // async test function on a thread-local executor. The test is considered done when that executor
        // has no more tasks to run. Therefore a test function may freely `spawn` as many futures as it wants.
        tokio::spawn(async move {
            sqlx::query(&format!("DROP DATABASE \"{}\"", database_name))
                .execute(&pg_pool)
                .await
                .unwrap();
        });
    }
}
