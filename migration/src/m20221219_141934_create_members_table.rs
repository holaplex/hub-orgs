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
                    .table(Members::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Members::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Members::UserId).uuid().not_null())
                    .col(ColumnDef::new(Members::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Members::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(ColumnDef::new(Members::RevokedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-members_organization_id-organizations")
                            .from(Members::Table, Members::OrganizationId)
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
                    .name("members_user_id_idx")
                    .table(Members::Table)
                    .col(Members::UserId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("members_organization_id_idx")
                    .table(Members::Table)
                    .col(Members::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Members {
    Table,
    Id,
    UserId,
    OrganizationId,
    CreatedAt,
    RevokedAt,
}

