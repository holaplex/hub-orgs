use async_graphql::{self, Context, Object, Result};
use sea_orm::prelude::*;

use crate::{entities::projects, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "ProjectQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<projects::Model>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        projects::Entity::find_by_id(id)
            .one(db.get())
            .await
            .map_err(Into::into)
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_project_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Option<projects::Model>> {
        self.project(ctx, id).await
    }
}
