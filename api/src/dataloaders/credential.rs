use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::DatabaseClient,
    entities::credentials::{Column, Entity, Model},
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
        let credentials = Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(&*self.db)
            .await?;

        Ok(credentials.iter().map(|c| (c.id, c.clone())).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrganizationId(pub Uuid);

#[async_trait]
impl DataLoader<OrganizationId> for Loader {
    type Error = FieldError;
    type Value = Vec<Model>;

    async fn load(
        &self,
        keys: &[OrganizationId],
    ) -> Result<HashMap<OrganizationId, Self::Value>, Self::Error> {
        let credentials = Entity::find()
            .filter(Column::OrganizationId.is_in(keys.iter().map(|o| o.0)))
            .all(&*self.db)
            .await?;

        let mut hashmap = HashMap::new();

        for c in credentials {
            hashmap
                .entry(OrganizationId(c.organization_id))
                .or_insert(Vec::new())
                .push(c);
        }

        Ok(hashmap)
    }
}
