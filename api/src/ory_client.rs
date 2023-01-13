use bytes::Bytes;
use clap::Parser;
use reqwest::{Client as ReqwestClient, Url};
use serde::Serialize;

use crate::prelude::*;

/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env, default_value = "localhost:4445")]
    ory_base_url: String,
    #[arg(long, env)]
    ory_auth_token: String,
}

pub struct OryClient {
    pub client: ReqwestClient,
    pub base_url: String,
    pub auth_token: String,
}

impl OryClient {
    pub(crate) fn new() -> Self {
        let Args {
            ory_base_url,
            ory_auth_token,
        } = Args::parse();

        Self {
            client: ReqwestClient::new(),
            base_url: ory_base_url,
            auth_token: ory_auth_token,
        }
    }

    pub async fn post(&self, endpoint: &str, body: impl Serialize) -> Result<Bytes> {
        let url = Url::parse(&format!("{}/admin/{endpoint}", self.base_url))?;

        self.client
            .post(url)
            .bearer_auth(self.auth_token.clone())
            .json(&body)
            .send()
            .await
            .context("failed to make post request to ory client")?
            .bytes()
            .await
            .context("failed to parse response to bytes")
    }
}
