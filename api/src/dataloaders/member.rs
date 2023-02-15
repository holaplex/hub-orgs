use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{db::Connection, entities::invites};

#[derive(Debug, Clone)]
pub struct InviteLoader {
    pub db: Connection,
}

impl InviteLoader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for InviteLoader {
    type Error = FieldError;
    type Value = invites::Model;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let invites = invites::Entity::find()
            .filter(invites::Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(invites.into_iter().map(|i| (i.id, i)).collect())
    }
}
