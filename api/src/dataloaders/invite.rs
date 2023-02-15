use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{db::Connection, entities::members};

#[derive(Debug, Clone)]
pub struct MemberLoader {
    pub db: Connection,
}

impl MemberLoader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DataLoader<Uuid> for MemberLoader {
    type Error = FieldError;
    type Value = members::Member;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let members = members::Entity::find()
            .filter(members::Column::InviteId.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(members
            .into_iter()
            .map(|m| (m.invite_id, m.into()))
            .collect())
    }
}
