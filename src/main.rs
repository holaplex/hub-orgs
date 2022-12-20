use std::sync::Arc;

use anyhow::{Context as AnyhowContext, Result};
use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_orgs::{
    dataloaders::OrganizationLoader, db::Connection, mutations::Mutation, prelude::*,
    queries::Query, UserID,
};
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};
use sea_orm::DatabaseConnection;

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[handler]
async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[handler]
async fn graphql_handler(
    Data(schema): Data<&AppSchema>,
    user_id: UserID,
    req: GraphQLRequest,
) -> GraphQLResponse {
    debug!("{:?}", user_id);

    schema.execute(req.0.data(user_id)).await.into()
}

pub struct Context {
    db: Arc<DatabaseConnection>,
    organization_loader: DataLoader<OrganizationLoader>,
}

impl Context {
    async fn new() -> Result<Self> {
        let db = Connection::new()
            .await
            .context("failed to get database connection")?
            .get();

        let organization_loader =
            DataLoader::new(OrganizationLoader::new(db.clone()), tokio::spawn);

        Ok(Self {
            db,
            organization_loader,
        })
    }
}
/// Builds the GraphQL Schema, attaching the Database to the context
async fn build_schema(ctx: Context) -> Result<AppSchema> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .data(ctx.db)
        .data(ctx.organization_loader)
        .finish();

    Ok(schema)
}

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
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

    let app_context = Context::new().await?;
    let schema = build_schema(app_context).await?;

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(
            Route::new()
                .at("/graphql", post(graphql_handler))
                .at("/playground", get(playground))
                .data(schema),
        )
        .await
        .map_err(Into::into)
}
