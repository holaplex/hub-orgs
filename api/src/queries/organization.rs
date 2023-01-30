use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder};

use crate::{entities::organizations, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "OrganizationQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn organizations(&self, ctx: &Context<'_>) -> Result<Vec<organizations::Model>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        organizations::Entity::find()
            .order_by_desc(organizations::Column::CreatedAt)
            .all(db.get())
            .await
            .map_err(Into::into)
    }
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn organization(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<organizations::Model>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        organizations::Entity::find_by_id(id)
            .one(db.get())
            .await
            .map_err(Into::into)
    }
}
