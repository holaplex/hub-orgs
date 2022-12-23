use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

use crate::m20221215_150612_create_organizations_table::Organizations;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Type)
                    .values([Status::Sent, Status::Accepted, Status::Revoked])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Invites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invites::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Invites::Email).string().not_null())
                    .col(
                        ColumnDef::new(Invites::Status)
                            .custom(Status::Type)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Invites::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Invites::CreatedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Invites::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(ColumnDef::new(Invites::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-invites_organization_id-organizations")
                            .from(Invites::Table, Invites::OrganizationId)
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
                    .name("invites_created_at_idx")
                    .table(Invites::Table)
                    .col(Invites::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("invities_email_idx")
                    .table(Invites::Table)
                    .col(Invites::Email)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("invities_organization_id_idx")
                    .table(Invites::Table)
                    .col(Invites::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invites::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Status::Type).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Invites {
    Table,
    Id,
    Email,
    Status,
    OrganizationId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

enum Status {
    Type,
    Sent,
    Accepted,
    Revoked,
}

impl Iden for Status {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Type => "invite_status",
            Self::Sent => "sent",
            Self::Accepted => "accepted",
            Self::Revoked => "revoked",
        })
        .unwrap();
    }
}
