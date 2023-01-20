use bytes::Bytes;
use clap::Parser;
use reqwest::{Client as ReqwestClient, Response, Url};
use serde::Serialize;

use crate::prelude::*;

/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env, default_value = "http://127.0.0.1:4445")]
    ory_base_url: String,
    #[arg(long, env, default_value = "")]
    ory_auth_token: String,
}

pub struct OryClient {
    pub client: ReqwestClient,
    pub base_url: String,
    pub auth_token: String,
}

impl OryClient {
    pub(crate) fn new() -> Self {
        if cfg!(debug_assertions) {
            dotenv::dotenv().ok();
        }

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

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn post(&self, endpoint: &str, body: impl Serialize) -> Result<Bytes> {
        let url = Url::parse(&format!("{}/admin", self.base_url))?.join(endpoint)?;

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

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn delete(&self, endpoint: &str) -> Result<Response> {
        let url = Url::parse(&format!("{}/admin", self.base_url))?.join(endpoint)?;

        let response = self
            .client
            .delete(url)
            .bearer_auth(self.auth_token.clone())
            .send()
            .await
            .context("failed to make post request to ory client")?;

        Ok(response)
    }
}
