use hub_core::{
    anyhow::Result,
    clap,
    prelude::*,
    reqwest,
    reqwest::{Response, Url},
};
use serde::Serialize;

/// Arguments for establishing a database connection
#[derive(Debug, clap::Args)]
pub struct OryArgs {
    #[arg(long, env, default_value = "http://127.0.0.1:4445")]
    ory_base_url: String,
    #[arg(long, env, default_value = "")]
    ory_auth_token: String,
}

#[derive(Clone, Debug)]
pub struct OryClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub auth_token: String,
}

impl OryClient {
    #[must_use]
    pub fn new(args: OryArgs) -> Self {
        let OryArgs {
            ory_base_url,
            ory_auth_token,
        } = args;

        Self {
            client: reqwest::Client::new(),
            base_url: ory_base_url,
            auth_token: ory_auth_token,
        }
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if response is an error
    pub async fn post<D: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Serialize,
    ) -> Result<D> {
        let url = Url::parse(&format!("{}/admin", self.base_url))?.join(endpoint)?;

        self.client
            .post(url)
            .bearer_auth(self.auth_token.clone())
            .json(&body)
            .send()
            .await
            .context("failed to make post request to ory client")?
            .json()
            .await
            .context("failed to parse response to bytes")
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if response is an error
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
