use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::{prelude::*, JoinType, QuerySelect};

use crate::{
    db::DatabaseClient,
    entities::{
        projects::{self, Model as Project},
        webhook_projects::{self, Model as WebhookProject},
    },
};

pub struct Loader {
    pub db: DatabaseClient,
}

impl Loader {
    #[must_use]
    pub fn new(db: DatabaseClient) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Vec<Project>;

    async fn load(&self, webhook_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let projects: Vec<(WebhookProject, Vec<Project>)> = webhook_projects::Entity::find()
            .select_with(projects::Entity)
            .join(
                JoinType::InnerJoin,
                webhook_projects::Relation::Projects.def(),
            )
            .filter(
                webhook_projects::Column::WebhookId
                    .is_in(webhook_ids.iter().map(ToOwned::to_owned)),
            )
            .all(&*self.db)
            .await?;

        Ok(projects
            .iter()
            .map(|(wp, projects)| (wp.webhook_id, projects.clone()))
            .collect())
    }
}
