use sea_orm::Statement;
use sea_orm_migration::{prelude::*, sea_orm::ConnectionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Organizations::Slug)
                            .custom(ColumnType::CharVarying)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        let sql = r#"
            ALTER TABLE organizations ADD CONSTRAINT slug_regexp_check CHECK (slug ~* '^[A-Za-z0-9-]+$');
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());

        manager.get_connection().execute(stmt).await.map(|_| ())?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .drop_column(Organizations::Slug)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum ColumnType {
    #[iden = "character varying(63)"]
    CharVarying,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Organizations {
    Table,
    Slug,
}
