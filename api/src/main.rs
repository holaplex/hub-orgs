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
mod queries;

mod prelude {
    pub use std::time::Duration;

    pub use anyhow::{anyhow, bail, Context, Result};
    pub use chrono::{DateTime, Utc};
    pub use clap::Parser;
    pub use log::debug;
}

use std::sync::Arc;

use anyhow::{anyhow, Context as AnyhowContext, Result};
use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use dataloaders::OrganizationLoader;
use db::Connection;
use mutations::Mutation;
use poem::{
    async_trait, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Html},
    EndpointExt, FromRequest, IntoResponse, Request, RequestBody, Route, Server,
};
use prelude::*;
use queries::Query;
use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct UserID(pub String);

impl From<String> for UserID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map(|v| Self(v.to_string()))
            .ok_or_else(|| anyhow!("X-USER-ID not provided in the request"))
            .map_err(Into::into);

        id
    }
}

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
/// # Errors
/// This function fails if ...
pub async fn build_schema(ctx: Context) -> Result<AppSchema> {
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

    let app_context = Context::new()
        .await
        .context("failed to build app context")?;
    let schema = build_schema(app_context)
        .await
        .context("failed to build schema")?;

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(
            Route::new()
                .at("/graphql", post(graphql_handler))
                .at("/playground", get(playground))
                .data(schema),
        )
        .await
        .context("failed to build graphql server")
}

#[cfg(test)]
mod test;