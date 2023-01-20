use std::sync::Arc;

use clap::Parser;
use webhooks::api::{Svix as SvixOpenapiClient, SvixOptions};

/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env, default_value = "http://127.0.0.1:8071")]
    svix_base_url: String,
    #[arg(long, env)]
    svix_auth_token: String,
}

pub type SvixClient = Arc<SvixOpenapiClient>;

#[derive(Clone)]
pub struct Client(SvixClient);

impl Client {
    pub(crate) fn new() -> Self {
        if cfg!(debug_assertions) {
            dotenv::dotenv().ok();
        }

        let Args {
            svix_base_url,
            svix_auth_token,
        } = Args::parse();

        let svix_options = SvixOptions {
            debug: true,
            server_url: Some(svix_base_url),
        };

        let svix_client = SvixOpenapiClient::new(svix_auth_token, Some(svix_options));

        Self(Arc::new(svix_client))
    }

    #[must_use]
    pub(crate) fn get(self) -> SvixClient {
        self.0
    }
}
