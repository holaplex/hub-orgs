use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use hub_core::{
    async_graphql::{
        extensions,
        http::{playground_source, GraphQLPlaygroundConfig},
        EmptySubscription, Schema,
    },
    db::Connection,
    prelude::info,
};

mod mutations;
mod queries;

use mutations::Mutation;
use queries::Query;
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> Result<AppSchema> {
    let db = Connection::new().await.unwrap().get();

    // todo! Shared struct instead of db

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(extensions::Logger)
        .data(db)
        .finish();

    Ok(schema)
}

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
}

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.dev").ok();
    } else {
        dotenv::dotenv().ok();
    }

    let schema = build_schema().await?;

    // todo! graphql routes and address as env variables
    info!("Playground: http://localhost:3000/graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql_handler))
            .service(
                web::resource("/graphql/playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
    .map_err(Into::into)
}
