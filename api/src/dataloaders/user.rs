use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::Connection,
    entities::{
        members::{Column as MColumn, Entity as MEntity, Member},
        owners::{Column, Entity, Owner},
    },
};

#[derive(Debug, Clone)]
pub struct MembersLoader {
    pub db: Connection,
}

impl MembersLoader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for MembersLoader {
    type Error = FieldError;
    type Value = Vec<Member>;

    async fn load(
        &self,
        organization_ids: &[Uuid],
    ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let members = MEntity::find()
            .filter(MColumn::OrganizationId.is_in(organization_ids.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        let mut hashmap = HashMap::new();

        for m in members {
            hashmap
                .entry(m.organization_id)
                .or_insert(Vec::new())
                .push(m.into());
        }

        Ok(hashmap)
    }
}

#[derive(Debug, Clone)]
pub struct OwnerLoader {
    pub db: Connection,
}

impl OwnerLoader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for OwnerLoader {
    type Error = FieldError;
    type Value = Owner;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let owners = Entity::find()
            .filter(Column::OrganizationId.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(owners
            .iter()
            .map(|o| (o.organization_id, (*o).into()))
            .collect())
    }
}
