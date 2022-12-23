use std::{collections::HashMap, sync::Arc};

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::entities::{
    members::{Column as MColumn, Entity as MEntity, Model as MModel},
    owners::{Column, Entity, Model},
};

pub struct MembersLoader {
    pub db: Arc<DatabaseConnection>,
}

impl MembersLoader {
    #[must_use]
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for MembersLoader {
    type Error = FieldError;
    type Value = Vec<MModel>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let members = MEntity::find()
            .filter(MColumn::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(&*self.db)
            .await?;

        Ok(members.iter().map(|m| (m.id, vec![m.clone()])).collect())
    }
}

pub struct OwnerLoader {
    pub db: Arc<DatabaseConnection>,
}

impl OwnerLoader {
    #[must_use]
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for OwnerLoader {
    type Error = FieldError;
    type Value = Model;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let owners = Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(&*self.db)
            .await?;

        Ok(owners.iter().map(|m| (m.id, m.clone())).collect())
    }
}
