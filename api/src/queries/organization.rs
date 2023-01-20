use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder};

use crate::{db::DatabaseClient, entities::organizations};
#[derive(Default)]
pub struct Query;

#[Object(name = "OrganizationQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn organizations(&self, ctx: &Context<'_>) -> Result<Vec<organizations::Model>> {
        let db = &**ctx.data::<DatabaseClient>()?;

        organizations::Entity::find()
            .order_by_desc(organizations::Column::CreatedAt)
            .all(db)
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
        id: uuid::Uuid,
    ) -> Result<Option<organizations::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        organizations::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Into::into)
    }
}
