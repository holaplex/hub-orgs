use std::{collections::HashMap, sync::Arc};

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::entities::organizations::{Column, Entity, Model};

pub struct Loader {
    pub db: Arc<DatabaseConnection>,
}

impl Loader {
    #[must_use]
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Model;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let orgs = Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(&*self.db)
            .await?;

        Ok(orgs.iter().map(|o| (o.id, o.clone())).collect())
    }
}
