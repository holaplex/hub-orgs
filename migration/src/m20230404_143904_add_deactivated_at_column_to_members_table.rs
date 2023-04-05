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
                    .add_column(ColumnDef::new(Members::DeactivatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Members::Table)
                    .drop_column(Members::DeactivatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Members {
    Table,
    DeactivatedAt,
}
