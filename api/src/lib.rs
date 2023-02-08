#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod apis;
pub mod db;
#[allow(clippy::pedantic)]
pub mod entities;
pub mod handlers;
pub mod ory_client;
pub mod svix_client;

use db::Connection;
use hub_core::{clap, prelude::*};
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

#[derive(Clone)]
pub struct AppState {
    pub connection: Connection,
    pub ory_client: OryClient,
    pub svix_client: Svix,
}

impl AppState {
    #[must_use]
    pub fn new(connection: Connection, ory_client: OryClient, svix_client: Svix) -> Self {
        Self {
            connection,
            ory_client,
            svix_client,
        }
    }
}
