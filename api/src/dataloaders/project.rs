use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::{prelude::*, JoinType, QuerySelect};

use crate::{
    db::DatabaseClient,
    entities::{
        project_credentials,
        projects::{self, Column, Entity, Model},
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
    type Value = Model;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let projects = Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(&*self.db)
            .await?;

        Ok(projects
            .iter()
            .map(|project| (project.id, project.clone()))
            .collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CredentialId(pub Uuid);

#[async_trait]
impl DataLoader<CredentialId> for Loader {
    type Error = FieldError;
    type Value = Vec<Model>;

    async fn load(
        &self,
        keys: &[CredentialId],
    ) -> Result<HashMap<CredentialId, Self::Value>, Self::Error> {
        let project_credentials: Vec<(project_credentials::Model, Vec<Model>)> =
            project_credentials::Entity::find()
                .select_with(projects::Entity)
                .join(
                    JoinType::InnerJoin,
                    project_credentials::Relation::Projects.def(),
                )
                .filter(project_credentials::Column::CredentialId.is_in(keys.iter().map(|c| c.0)))
                .all(&*self.db)
                .await?;

        Ok(project_credentials
            .iter()
            .map(|(credential, projects)| (CredentialId(credential.id), projects.clone()))
            .collect())
    }
}
