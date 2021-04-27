use std::convert::Infallible;

use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptyMutation;
use async_graphql::Schema;

use crate::repository::Repository;
use crate::schema::query::Query;
use crate::schema::subscription::Subscription;

///
/// Start a web server providing the /graphql endpoint plus a playground
///
pub async fn serve(pg_pool: sqlx::PgPool) {
    let schema = Schema::build(Query, EmptyMutation, Subscription).finish();

    let graphql_post = warp::path("/graphql")
        .and(warp::post())
        .and(async_graphql_warp::graphql(schema))
        .and(warp::any().map(move || pg_pool.clone()))
        .and_then(execute_graphql);

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let routes = graphql_post
        .or(graphql_playground)
        .recover(|err: Rejection| async move {
            if let Some(async_graphql_warp::BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn execute_graphql(
    (schema, mut request): (
        Schema<Query, EmptyMutation, Subscription>,
        async_graphql::Request,
    ),
    pg_pool: sqlx::PgPool,
) -> Result<async_graphql_warp::Response, Infallible> {
    // Set up the GraphQL context:
    request = request.data(Repository::new(pg_pool));

    let response = schema.execute(request).await;

    // graphql never 'fails' at the HTTP status-code level:
    Ok(async_graphql_warp::Response::from(response))
}
