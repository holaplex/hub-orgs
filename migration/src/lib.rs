pub use sea_orm_migration::prelude::*;

mod m20221215_150612_create_organizations_table;
mod m20221217_004651_owners_table;
mod m20221219_134917_create_projects_table;
mod m20221219_141929_create_invites_table;
mod m20221219_141934_create_members_table;
mod m20230113_044004_create_credentials_table;
mod m20230113_044805_create_project_credentials_table;
mod m20230121_045004_add_svix_app_id_column_to_organizations_table;
mod m20230124_165007_webhooks_table;
mod m20230124_171112_webhook_projects_table;
mod m20230202_180215_add_slug_to_organizations;
mod m20230208_144934_drop_slug_from_organizations;
mod m20230215_092536_add_invite_id_to_members;

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
            Box::new(m20230121_045004_add_svix_app_id_column_to_organizations_table::Migration),
            Box::new(m20230124_165007_webhooks_table::Migration),
            Box::new(m20230124_171112_webhook_projects_table::Migration),
            Box::new(m20230202_180215_add_slug_to_organizations::Migration),
            Box::new(m20230208_144934_drop_slug_from_organizations::Migration),
            Box::new(m20230215_092536_add_invite_id_to_members::Migration),
        ]
    }
}
