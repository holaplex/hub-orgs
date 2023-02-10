use async_graphql::{Context, Object, Result};
use sea_orm::prelude::*;

use crate::{entities::organizations, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "OrganizationQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn organization(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<organizations::Organization>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        Ok(organizations::Entity::find_by_id(id)
            .one(db.get())
            .await?
            .map(Into::into))
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_organization_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Option<organizations::Organization>> {
        self.organization(ctx, id).await
    }
}
