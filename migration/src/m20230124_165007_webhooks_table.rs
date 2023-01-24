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
                    .table(Webhooks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Webhooks::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Webhooks::EndpointId).string().not_null())
                    .col(ColumnDef::new(Webhooks::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Webhooks::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(ColumnDef::new(Webhooks::UpdatedAt).timestamp())
                    .col(ColumnDef::new(Webhooks::CreatedBy).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-webhooks_organization_id")
                            .from(Webhooks::Table, Webhooks::OrganizationId)
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
                    .name("webhooks_endpoint_id_idx")
                    .table(Webhooks::Table)
                    .col(Webhooks::EndpointId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("webhooks_organization_id_idx")
                    .table(Webhooks::Table)
                    .col(Webhooks::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("webhooks_created_at_idx")
                    .table(Webhooks::Table)
                    .col(Webhooks::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("Webhooks_user_Id_idx")
                    .table(Webhooks::Table)
                    .col(Webhooks::CreatedBy)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Webhooks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Webhooks {
    Table,
    Id,
    EndpointId,
    OrganizationId,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
}
