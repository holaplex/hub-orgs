#[allow(clippy::all)]
pub use sea_orm_migration::prelude::*;

mod m20221212_170906_organizations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20221212_170906_organizations::Migration)]
    }
}
