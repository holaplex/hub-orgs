use sea_orm_migration::prelude::*;

use crate::m20221215_150612_create_organizations_table::Organizations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Credentials::Id).uuid().primary_key())
                    .col(ColumnDef::new(Credentials::ClientId).string().not_null())
                    .col(
                        ColumnDef::new(Credentials::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Credentials::Name).string().not_null())
                    .col(
                        ColumnDef::new(Credentials::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(ColumnDef::new(Credentials::CreatedBy).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("credentials_organization_id_fk")
                            .from(Credentials::Table, Credentials::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credentials_org-id_idx")
                    .table(Credentials::Table)
                    .col(Credentials::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credentials_created-by_idx")
                    .table(Credentials::Table)
                    .col(Credentials::CreatedBy)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("credentials_name_idx")
                    .table(Credentials::Table)
                    .col(Credentials::Name)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Credentials {
    Table,
    Id,
    ClientId,
    OrganizationId,
    Name,
    CreatedAt,
    CreatedBy,
}
