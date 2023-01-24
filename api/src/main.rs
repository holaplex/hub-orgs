#![deny(
    clippy::disallowed_methods,
    clippy::suspicious,
    clippy::style,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

mod dataloaders;
mod db;
mod entities;
mod mutations;
mod ory_client;
mod queries;

use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use dataloaders::{
    CredentialLoader, MembersLoader, OrganizationLoader, OwnerLoader, ProjectCredentialsLoader,
    ProjectLoader,
};
use db::Connection;
use hub_core::{clap, prelude::*, tokio, uuid::Uuid};
use mutations::Mutation;
use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html},
    EndpointExt, FromRequest, IntoResponse, Request, RequestBody, Route, Server,
};
use queries::Query;
use sea_orm::DatabaseConnection;

use crate::ory_client::OryClient;

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3002)]
    port: u16,

    #[command(flatten)]
    db: db::DbArgs,

    #[command(flatten)]
    ory: ory_client::OryArgs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserID(Option<Uuid>);

impl TryFrom<&str> for UserID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = Uuid::from_str(value)?;

        Ok(Self(Some(id)))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map_or(Ok(Self(None)), Self::try_from)?;

        Ok(id)
    }
}

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[handler]
fn health() {}

#[handler]
fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[handler]
async fn graphql_handler(
    Data(schema): Data<&AppSchema>,
    user_id: UserID,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.0.data(user_id)).await.into()
}

#[allow(missing_debug_implementations)]
pub struct Context {
    db: Arc<DatabaseConnection>,
    organization_loader: DataLoader<OrganizationLoader>,
    members_loader: DataLoader<MembersLoader>,
    owner_loader: DataLoader<OwnerLoader>,
    project_credentials_loader: DataLoader<ProjectCredentialsLoader>,
    project_loader: DataLoader<ProjectLoader>,
    credential_loader: DataLoader<CredentialLoader>,
    ory_client: OryClient,
}

impl Context {
    async fn new(db_args: db::DbArgs, ory_args: ory_client::OryArgs) -> Result<Self> {
        let db = Connection::new(db_args)
            .await
            .context("failed to get database connection")?
            .get();

        let organization_loader =
            DataLoader::new(OrganizationLoader::new(db.clone()), tokio::spawn);
        let members_loader = DataLoader::new(MembersLoader::new(db.clone()), tokio::spawn);
        let owner_loader = DataLoader::new(OwnerLoader::new(db.clone()), tokio::spawn);
        let project_credentials_loader =
            DataLoader::new(ProjectCredentialsLoader::new(db.clone()), tokio::spawn);
        let project_loader = DataLoader::new(ProjectLoader::new(db.clone()), tokio::spawn);
        let credential_loader = DataLoader::new(CredentialLoader::new(db.clone()), tokio::spawn);
        let ory_client = OryClient::new(ory_args);

        Ok(Self {
            db,
            organization_loader,
            members_loader,
            owner_loader,
            project_credentials_loader,
            project_loader,
            credential_loader,
            ory_client,
        })
    }
}
/// Builds the GraphQL Schema, attaching the Database to the context
/// # Errors
/// This function fails if ...
pub fn build_schema(ctx: Context) -> Result<AppSchema> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .data(ctx.db)
        .data(ctx.organization_loader)
        .data(ctx.members_loader)
        .data(ctx.owner_loader)
        .data(ctx.project_credentials_loader)
        .data(ctx.project_loader)
        .data(ctx.credential_loader)
        .data(ctx.ory_client)
        .finish();

    Ok(schema)
}

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-orgs",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, db, ory } = args;

        common.rt.block_on(async move {
            let app_context = Context::new(db, ory)
                .await
                .context("failed to build app context")?;
            let schema = build_schema(app_context)
                .context("failed to build schema")?;

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at("/graphql", post(graphql_handler))
                        .at("/playground", get(playground))
                        .at("/health", get(health))
                        .data(schema),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}

#[cfg(test)]
mod test;
