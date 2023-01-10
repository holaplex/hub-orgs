use std::sync::Arc;

use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};

use crate::entities::projects;

#[derive(Default)]
pub struct Query;

#[Object(name = "ProjectQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 25)] limit: u64,
        #[graphql(default = 0)] offset: u64,
    ) -> Result<Vec<projects::Model>> {
        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;

        projects::Entity::find()
            .order_by_desc(projects::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await
            .map_err(Into::into)
    }
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn project(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<Option<projects::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        projects::Entity::find_by_id(id)
            .one(db)
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
        #[graphql(key)] id: uuid::Uuid,
    ) -> Result<Option<projects::Model>> {
        self.project(ctx, id).await
    }
}
