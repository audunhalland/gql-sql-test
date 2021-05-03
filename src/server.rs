use std::convert::Infallible;

use async_graphql_warp::graphql_subscription;
use warp::http::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;

use crate::bus::EventBus;
use crate::repository::Repository;
use crate::schema::mutation::Mutation;
use crate::schema::query::Query;
use crate::schema::subscription::Subscription;

const SERVER_PORT: u16 = 8000;

///
/// Start a web server providing the /graphql endpoint plus a playground.
///
/// The server runs as long as its future is polled by the executor.
/// The server is a future that never completes.
///
pub async fn serve(pg_pool: sqlx::PgPool) {
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(Repository::new(pg_pool))
        .data(EventBus::new())
        .finish();

    let graphql_post = warp::path!("graphql")
        .and(warp::post())
        // The followig filter has an Extract type, and provides the first parameter to `execute_graphql`:
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |(schema, request): (Schema<_, _, _>, async_graphql::Request)| async move {
                let response = schema.execute(request).await;

                // graphql never 'fails' at the HTTP status-code level:
                Ok::<_, Infallible>(async_graphql_warp::Response::from(response))
            },
        );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/graphql")
                    .subscription_endpoint(&format!("ws://localhost:{}", SERVER_PORT)),
            ))
    });

    let routes = graphql_post
        .or(graphql_subscription(schema))
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

    warp::serve(routes).run(([0, 0, 0, 0], SERVER_PORT)).await;
}
