use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::Connection,
    entities::project_credentials::{Column, Entity, Model as ProjectCredential},
};

#[derive(Debug, Clone)]
pub struct Loader {
    pub db: Connection,
}

impl Loader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Vec<ProjectCredential>;

    async fn load(
        &self,
        credential_ids: &[Uuid],
    ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let project_credentials = Entity::find()
            .filter(Column::CredentialId.is_in(credential_ids.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        let mut hashmap = HashMap::new();

        for pc in project_credentials {
            hashmap
                .entry(pc.credential_id)
                .or_insert(Vec::new())
                .push(pc);
        }

        Ok(hashmap)
    }
}
