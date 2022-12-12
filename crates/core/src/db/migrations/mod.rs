pub use sea_orm_migration::prelude::*;

mod m20221210_190223_create_organizations_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20221210_190223_create_organizations_table::Migration,
        )]
    }
}
