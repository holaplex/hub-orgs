use async_graphql::{
    extensions,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use hub_orgs::{db::Connection, mutations::Mutation, prelude::*, queries::Query};
use log::info;
use poem::{get, handler, listener::TcpListener, post, web::Html, IntoResponse, Route, Server};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> Result<AppSchema> {
    let db = Connection::new()
        .await
        .context("failed to get db connection")?;

    // todo! Shared struct instead of db

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(extensions::Logger)
        .data(db.get())
        .finish();

    Ok(schema)
}

#[handler]
async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.dev").ok();
    } else {
        dotenv::dotenv().ok();
    }

    env_logger::builder()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .parse_default_env()
        .init();

    let schema = build_schema().await?;

    // todo! graphql routes and address as env variables
    // core crate server options
    info!("Playground: http://localhost:3001/graphql/playground");

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(
            Route::new()
                .at("/graphql", post(GraphQL::new(schema)))
                .at("/playground", get(playground)),
        )
        .await
        .map_err(Into::into)
}
