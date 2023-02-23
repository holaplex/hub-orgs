use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectCredentials::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Credentials::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
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
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectCredentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectCredentials::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra(" default gen_random_uuid()".to_string()),
                    )
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
}

/// Learn more at https://docs.rs/sea-query#iden
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

#[derive(Iden)]
enum ProjectCredentials {
    Table,
    Id,
    CredentialId,
    ProjectId,
    CreatedAt,
    CreatedBy,
}

#[derive(Iden)]
enum Projects {
    Table,
    Id,
}

#[derive(Iden)]
enum Organizations {
    Table,
    Id,
}
