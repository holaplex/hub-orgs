use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

#[tokio::test]

async fn test_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let opts = sea_orm::ConnectOptions::new(url);
    let db = sea_orm::Database::connect(opts).await?;

    let applied = migration::Migrator::up(&db, None).await;

    assert!(applied.is_ok());

    let refresh = migration::Migrator::refresh(&db).await;

    assert!(refresh.is_ok());

    Ok(())
}
