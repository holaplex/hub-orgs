use async_graphql::{Context, Object, Result};
use hub_core::uuid::Uuid;
use sea_orm::prelude::*;

use crate::{
    entities::invites::{Column, Entity, Model},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "InviteQuery")]
impl Query {
    /// Retrieve a member invitation by its ID.
    async fn invite(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Model>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let conn = db.get();

        Entity::find()
            .filter(Column::Id.eq(id))
            .one(conn)
            .await
            .map_err(Into::into)
    }
}
