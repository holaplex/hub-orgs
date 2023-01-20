use std::sync::Arc;

pub use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::prelude::*;

/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env, default_value = "500")]
    max_connections: u32,
    #[arg(long, env, default_value = "60")]
    connection_timeout: u64,
    #[arg(long, env, default_value = "10")]
    acquire_timeout: u64,
    #[arg(long, env, default_value = "60")]
    idle_timeout: u64,
    #[arg(long, env)]
    database_url: String,
}

pub type DatabaseClient = Arc<DatabaseConnection>;

#[derive(Clone)]
pub struct Connection(DatabaseClient);

impl Connection {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn new() -> Result<Self> {
        let Args {
            max_connections,
            connection_timeout,
            acquire_timeout,
            idle_timeout,
            database_url,
        } = Args::parse();

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

    pub fn get(self) -> DatabaseClient {
        self.0
    }
}
