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
                    .table(Owners::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Owners::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Owners::UserId).uuid().not_null())
                    .col(ColumnDef::new(Owners::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Owners::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-owners_organization-organizations")
                            .from(Owners::Table, Owners::OrganizationId)
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
                    .name("owners_organization_id_idx")
                    .table(Owners::Table)
                    .col(Owners::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("owners_user_Id_idx")
                    .table(Owners::Table)
                    .col(Owners::UserId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Owners::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Owners {
    Table,
    Id,
    OrganizationId,
    UserId,
    CreatedAt,
}
