pub mod db;
pub mod entities;
pub mod mutations;
pub mod queries;

#[path = "../migration/mod.rs"]
mod migration;

pub mod prelude {

    pub use std::time::Duration;

    pub use anyhow::{Context, Result};
    pub use chrono::{DateTime, Utc};
    pub use clap::Parser;
    pub use log::debug;
}
