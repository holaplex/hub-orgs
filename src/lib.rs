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

pub mod dataloaders;
pub mod db;
#[allow(clippy::pedantic)]
pub mod entities;
pub mod mutations;
pub mod queries;
pub mod prelude {

    pub use std::time::Duration;

    pub use anyhow::{anyhow, bail, Context, Result};
    pub use chrono::{DateTime, Utc};
    pub use clap::Parser;
    pub use log::debug;
}

use anyhow::anyhow;
use poem::{async_trait, FromRequest, Request, RequestBody, Result};

#[derive(Debug)]
pub struct UserID(pub String);

impl From<String> for UserID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
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
