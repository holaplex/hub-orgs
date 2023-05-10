use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

use crate::{
    m20221215_150612_create_organizations_table::Organizations,
    m20221217_004651_owners_table::Owners, m20221219_134917_create_projects_table::Projects,
    m20221219_141929_create_invites_table::Invites, m20221219_141934_create_members_table::Members,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"alter database orgs set timezone to 'utc' ;"#.to_string(),
        );

        db.execute(stmt).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("deactivated_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Owners::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("deactivated_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invites::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invites::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("revoked_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("deactivated_at")).timestamp_with_time_zone(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
