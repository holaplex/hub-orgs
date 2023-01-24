use std::{sync::Arc, time::Duration};

use hub_core::{clap, prelude::*};
pub use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Arguments for establishing a database connection
#[derive(Debug, clap::Args)]
pub struct DbArgs {
    #[arg(long, env, default_value_t = 500)]
    max_connections: u32,
    #[arg(long, env, default_value_t = 60)]
    connection_timeout: u64,
    #[arg(long, env, default_value_t = 10)]
    acquire_timeout: u64,
    #[arg(long, env, default_value_t = 60)]
    idle_timeout: u64,
    #[arg(long, env)]
    database_url: String,
}

pub struct Connection(Arc<DatabaseConnection>);

impl Connection {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn new(args: DbArgs) -> Result<Self> {
        let DbArgs {
            max_connections,
            connection_timeout,
            acquire_timeout,
            idle_timeout,
            database_url,
        } = args;

        let options = ConnectOptions::new(database_url)
            .max_connections(max_connections)
            .connect_timeout(Duration::from_secs(connection_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .clone();

        let db = Database::connect(options)
            .await
            .context("failed to get database connection")?;

        Ok(Self(Arc::new(db)))
    }

    #[must_use]

    pub fn get(self) -> Arc<DatabaseConnection> {
        self.0
    }
}
