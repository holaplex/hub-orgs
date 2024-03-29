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
mod m20230223_114331_drop_credentials_and_project_credentials_tables;
mod m20230301_000808_delete_webhook_projects_table;
mod m20230301_000812_delete_webhooks_table;
mod m20230301_004428_remove_svix_app_id_column;
mod m20230306_093249_add_profile_image_url_to_organizations;
mod m20230306_093255_add_profile_image_url_to_projects;
mod m20230404_143904_add_deactivated_at_column_to_members_table;
mod m20230510_154238_set_default_timestamp_to_utc;
mod m20230518_034509_set_default_timestamp;
mod m20230519_050922_change_email_datatype_to_citext;
mod m20230622_150710_remove_unique_org_name_constraint;

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
            Box::new(m20230223_114331_drop_credentials_and_project_credentials_tables::Migration),
            Box::new(m20230301_000808_delete_webhook_projects_table::Migration),
            Box::new(m20230301_000812_delete_webhooks_table::Migration),
            Box::new(m20230301_004428_remove_svix_app_id_column::Migration),
            Box::new(m20230306_093249_add_profile_image_url_to_organizations::Migration),
            Box::new(m20230306_093255_add_profile_image_url_to_projects::Migration),
            Box::new(m20230404_143904_add_deactivated_at_column_to_members_table::Migration),
            Box::new(m20230510_154238_set_default_timestamp_to_utc::Migration),
            Box::new(m20230518_034509_set_default_timestamp::Migration),
            Box::new(m20230519_050922_change_email_datatype_to_citext::Migration),
            Box::new(m20230622_150710_remove_unique_org_name_constraint::Migration),
        ]
    }
}
