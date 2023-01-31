use hub_core::clap;
use svix::api::{Svix, SvixOptions};

/// Arguments for establishing a database connection
#[derive(Debug, clap::Args)]
pub struct SvixArgs {
    #[arg(long, env, default_value = "http://127.0.0.1:8071")]
    svix_base_url: String,
    #[arg(long, env)]
    svix_auth_token: String,
}

impl SvixArgs {
    #[must_use]
    pub fn build_client(&self) -> Svix {
        let SvixArgs {
            svix_base_url,
            svix_auth_token,
        } = self;

        let svix_options = SvixOptions {
            debug: true,
            server_url: Some(svix_base_url.to_string()),
        };

        Svix::new(svix_auth_token.to_string(), Some(svix_options))
    }
}
