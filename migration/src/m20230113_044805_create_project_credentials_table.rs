use sea_orm_migration::prelude::*;

use crate::{
    m20221219_134917_create_projects_table::Projects,
    m20230113_044004_create_credentials_table::Credentials,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectCredentials::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProjectCredentials::Id).uuid().primary_key())
                    .col(
                        ColumnDef::new(ProjectCredentials::CredentialId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectCredentials::ProjectId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectCredentials::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .col(
                        ColumnDef::new(ProjectCredentials::CreatedBy)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_credentials-credential-id_fk")
                            .from(ProjectCredentials::Table, ProjectCredentials::CredentialId)
                            .to(Credentials::Table, Credentials::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("project_credentials-project_id_fk")
                            .from(ProjectCredentials::Table, ProjectCredentials::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("project_credentials_credential-id_idx")
                    .table(ProjectCredentials::Table)
                    .col(ProjectCredentials::CredentialId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("project_credentials_project-id_idx")
                    .table(ProjectCredentials::Table)
                    .col(ProjectCredentials::ProjectId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("project_credentials_createdby_idx")
                    .table(ProjectCredentials::Table)
                    .col(ProjectCredentials::CreatedBy)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectCredentials::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProjectCredentials {
    Table,
    Id,
    CredentialId,
    ProjectId,
    CreatedAt,
    CreatedBy,
}
