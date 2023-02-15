use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Members::InviteId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-members_invite_id-invites")
                            .from_tbl(Members::Table)
                            .from_col(Members::InviteId)
                            .to_tbl(Invites::Table)
                            .to_col(Invites::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .drop_column(Members::InviteId)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Members {
    Table,
    InviteId,
}

#[derive(Iden)]
enum Invites {
    Table,
    Id,
}
