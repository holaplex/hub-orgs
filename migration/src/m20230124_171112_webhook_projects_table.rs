use sea_orm_migration::prelude::*;

use crate::{
    m20221219_134917_create_projects_table::Projects, m20230124_165007_webhooks_table::Webhooks,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WebhookProjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WebhookProjects::WebhookId).uuid().not_null())
                    .col(ColumnDef::new(WebhookProjects::ProjectId).uuid().not_null())
                    .col(
                        ColumnDef::new(WebhookProjects::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .primary_key(
                        Index::create()
                            .col(WebhookProjects::WebhookId)
                            .col(WebhookProjects::ProjectId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-webhook-projects_projectid")
                            .from(WebhookProjects::Table, WebhookProjects::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-webhook-projects_webhookid")
                            .from(WebhookProjects::Table, WebhookProjects::WebhookId)
                            .to(Webhooks::Table, Webhooks::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("webhook-projects_webhook_id_idx")
                    .table(WebhookProjects::Table)
                    .col(WebhookProjects::WebhookId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("webhook-projects_project_id_idx")
                    .table(WebhookProjects::Table)
                    .col(WebhookProjects::ProjectId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("webhook-projects_created_at_idx")
                    .table(WebhookProjects::Table)
                    .col(WebhookProjects::CreatedAt)
                    .index_type(IndexType::BTree)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WebhookProjects::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum WebhookProjects {
    Table,
    WebhookId,
    ProjectId,
    CreatedAt,
}
