use async_graphql::{self, Context, Object, Result};
use sea_orm::prelude::*;

use crate::{
    entities::projects::{self, Project},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "ProjectQuery")]
impl Query {
    /// Query a project by it's ID, this query returns `null` if the project does not exist.
    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Project>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        projects::Entity::find_by_id(id)
            .one(db.get())
            .await
            .map_err(Into::into)
            .map(|p| p.map(Into::into))
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
    ) -> Result<Option<Project>> {
        self.project(ctx, id).await
    }
}
