//!

#![deny(
    clippy::pedantic,
    clippy::match_wildcard_for_single_variants,
    clippy::redundant_closure_for_method_calls,
    clippy::cargo
)]
#![warn(
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::suspicious,
    clippy::correctness,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::if_not_else,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::option_if_let_else,
    clippy::match_same_arms,
    clippy::default_trait_access,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::explicit_iter_loop,
    clippy::too_many_lines,
    clippy::cast_sign_loss,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::use_self,
    clippy::needless_borrow,
    clippy::redundant_pub_crate,
    clippy::useless_let_if_seq,
    // missing_docs,
    clippy::upper_case_acronyms
)]
#![forbid(unsafe_code)]
#![allow(clippy::unused_async)]

mod dataloaders;
mod db;
#[allow(clippy::pedantic)]
mod entities;
mod mutations;
mod ory_client;
mod queries;
mod svix_client;

mod prelude {
    pub use std::{ops::Deref, str::FromStr, sync::Arc, time::Duration};

    pub use anyhow::{anyhow, bail, Context as AnyhowContext, Result};
    pub use chrono::{DateTime, Utc};
    pub use clap::Parser;
    pub use log::debug;
}

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
use db::{Connection, DatabaseClient};
use mutations::Mutation;
use ory_client::OryClient;
use poem::{
    async_trait, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html},
    EndpointExt, FromRequest, IntoResponse, Request, RequestBody, Route, Server,
};
use prelude::*;
use queries::Query;
use svix_client::SvixClient;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long, env, default_value = "3002")]
    port: u16,
}

#[derive(Debug)]
pub struct UserID(Option<uuid::Uuid>);

impl TryFrom<&str> for UserID {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = uuid::Uuid::from_str(value)?;

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
async fn health() {}

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
    schema.execute(req.0.data(user_id)).await.into()
}

pub struct Context {
    db: DatabaseClient,
    organization_loader: DataLoader<OrganizationLoader>,
    members_loader: DataLoader<MembersLoader>,
    owner_loader: DataLoader<OwnerLoader>,
    project_credentials_loader: DataLoader<ProjectCredentialsLoader>,
    project_loader: DataLoader<ProjectLoader>,
    credential_loader: DataLoader<CredentialLoader>,
    ory_client: OryClient,
    svix_client: SvixClient,
}

impl Context {
    async fn new() -> Result<Self> {
        let db = Connection::new()
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
        let ory_client = OryClient::new();
        let svix_client = svix_client::Client::new().get();

        Ok(Self {
            db,
            organization_loader,
            members_loader,
            owner_loader,
            project_credentials_loader,
            project_loader,
            credential_loader,
            ory_client,
            svix_client,
        })
    }
}
/// Builds the GraphQL Schema, attaching the Database to the context
/// # Errors
/// This function fails if ...
pub async fn build_schema(ctx: Context) -> Result<AppSchema> {
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
        .data(ctx.svix_client)
        .finish();

    Ok(schema)
}

#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().ok();
    }

    let Args { port } = Args::parse();

    env_logger::builder()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .parse_default_env()
        .init();

    let app_context = Context::new()
        .await
        .context("failed to build app context")?;
    let schema = build_schema(app_context)
        .await
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
}

#[cfg(test)]
mod test;
