use std::time::Duration;

pub use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub use sea_orm_migration::prelude::*;
pub mod entities;
pub mod migrations;
pub mod mutation;
pub mod query;
use clap::Parser;

use crate::prelude::*;
/// Arguments for establishing a database connection
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, env, default_value = "500")]
    max_connections: u32,
    #[arg(long, env, default_value = "60")]
    connection_timeout: u64,
    #[arg(long, env, default_value = "60")]
    acquire_timeout: u64,
    #[arg(long, env, default_value = "60")]
    idle_timeout: u64,
    #[arg(long, env)]
    database_url: String,
}

pub struct Connection(DatabaseConnection);

impl Connection {
    pub async fn new() -> Result<Connection> {
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

        debug!("running migrations..");
        cli::run_cli(migrations::Migrator).await;

        Ok(Self(db))
    }

    pub fn get(self) -> DatabaseConnection {
        self.0
    }
}
