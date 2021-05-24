mod test_db;

use test_db::create_test_db;

use gql_sql_test::repository::Repository;

#[tokio::test]
async fn should_get_empty_todo_list_on_empty_database() {
    let todo_items = Repository::new(create_test_db().await)
        .list_todo_items(None, 0..10)
        .await
        .unwrap();

    assert_eq!(todo_items, vec![]);
}

#[tokio::test]
async fn should_insert_a_new_todo_item_and_then_fetch_it() {
    let repository = Repository::new(create_test_db().await);

    let inserted_todo_item = repository.insert_todo_item("foobar").await.unwrap();
    let todo_items = repository.list_todo_items(None, 0..10).await.unwrap();

    assert_eq!(todo_items, vec![inserted_todo_item]);
}

#[tokio::test]
async fn should_filter_todo_items_on_id() {
    let repository = Repository::new(create_test_db().await);

    let foo = repository.insert_todo_item("foo").await.unwrap();
    let _ = repository.insert_todo_item("bar").await.unwrap();

    let todo_items = repository
        .list_todo_items(Some(&[foo.id]), 0..1)
        .await
        .unwrap();

    assert_eq!(todo_items[0].id, foo.id);
}

#[tokio::test]
async fn should_set_item_to_done() {
    let repository = Repository::new(create_test_db().await);

    let item = repository.insert_todo_item("foo").await.unwrap();
    assert_eq!(item.done, false);

    let success = repository.set_done(item.id).await.unwrap();
    assert!(success);

    let items = repository.list_todo_items(None, 0..1).await.unwrap();
    assert_eq!(items[0].done, true);
}
