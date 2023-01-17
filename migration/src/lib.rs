#[allow(clippy::all)]
pub use sea_orm_migration::prelude::*;

mod m20221215_150612_create_organizations_table;
mod m20221217_004651_owners_table;
mod m20221219_134917_create_projects_table;
mod m20221219_141929_create_invites_table;
mod m20221219_141934_create_members_table;
mod m20230113_044004_create_credentials_table;
mod m20230113_044805_create_project_credentials_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221215_150612_create_organizations_table::Migration),
            Box::new(m20221217_004651_owners_table::Migration),
            Box::new(m20221219_134917_create_projects_table::Migration),
            Box::new(m20221219_141929_create_invites_table::Migration),
            Box::new(m20221219_141934_create_members_table::Migration),
            Box::new(m20230113_044004_create_credentials_table::Migration),
            Box::new(m20230113_044805_create_project_credentials_table::Migration),
        ]
    }
}
