#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod dataloaders;
pub mod db;
#[allow(clippy::pedantic)]
pub mod entities;
pub mod handlers;
pub mod mutations;
pub mod ory_client;
pub mod queries;
pub mod svix_client;

use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    EmptySubscription, Schema,
};
use dataloaders::{
    CredentialLoader, InviteMemberLoader, MemberInviteLoader, MembersLoader, OrganizationLoader,
    OwnerLoader, ProjectCredentialsLoader, ProjectLoader,
};
use db::Connection;
use hub_core::{
    anyhow::{Error, Result},
    clap,
    prelude::*,
    tokio,
    uuid::Uuid,
};
use mutations::Mutation;
use poem::{async_trait, FromRequest, Request, RequestBody};
use queries::Query;
use svix::api::Svix;

use crate::ory_client::OryClient;

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3002)]
    pub port: u16,

    #[command(flatten)]
    pub db: db::DbArgs,

    #[command(flatten)]
    pub ory: ory_client::OryArgs,

    #[command(flatten)]
    pub svix: svix_client::SvixArgs,
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct UserEmail(Option<String>);

#[async_trait]
impl<'a> FromRequest<'a> for UserEmail {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-EMAIL")
            .and_then(|value| value.to_str().ok())
            .map(std::string::ToString::to_string);

        Ok(Self(id))
    }
}

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub connection: Connection,
    pub ory_client: OryClient,
    pub svix_client: Svix,
}

impl AppState {
    #[must_use]
    pub fn new(
        schema: AppSchema,
        connection: Connection,
        ory_client: OryClient,
        svix_client: Svix,
    ) -> Self {
        Self {
            schema,
            connection,
            ory_client,
            svix_client,
        }
    }
}

pub struct AppContext {
    pub db: Connection,
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
    pub organization_loader: DataLoader<OrganizationLoader>,
    pub members_loader: DataLoader<MembersLoader>,
    pub owner_loader: DataLoader<OwnerLoader>,
    pub project_credentials_loader: DataLoader<ProjectCredentialsLoader>,
    pub project_loader: DataLoader<ProjectLoader>,
    pub credential_loader: DataLoader<CredentialLoader>,
    pub member_invite_loader: DataLoader<MemberInviteLoader>,
    pub invite_member_loader: DataLoader<InviteMemberLoader>,
}

impl AppContext {
    pub fn new(db: Connection, user_id: Option<Uuid>, user_email: Option<String>) -> Self {
        let organization_loader =
            DataLoader::new(OrganizationLoader::new(db.clone()), tokio::spawn);
        let members_loader = DataLoader::new(MembersLoader::new(db.clone()), tokio::spawn);
        let owner_loader = DataLoader::new(OwnerLoader::new(db.clone()), tokio::spawn);
        let project_credentials_loader =
            DataLoader::new(ProjectCredentialsLoader::new(db.clone()), tokio::spawn);
        let project_loader = DataLoader::new(ProjectLoader::new(db.clone()), tokio::spawn);
        let credential_loader = DataLoader::new(CredentialLoader::new(db.clone()), tokio::spawn);
        let member_invite_loader =
            DataLoader::new(MemberInviteLoader::new(db.clone()), tokio::spawn);
        let invite_member_loader =
            DataLoader::new(InviteMemberLoader::new(db.clone()), tokio::spawn);

        Self {
            db,
            user_id,
            user_email,
            organization_loader,
            members_loader,
            owner_loader,
            project_credentials_loader,
            project_loader,
            credential_loader,
            member_invite_loader,
            invite_member_loader,
        }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
#[must_use]
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
