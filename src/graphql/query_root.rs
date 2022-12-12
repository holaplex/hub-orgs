use async_graphql::{self, Context, Object, Result};
use sea_orm::DatabaseConnection;

use crate::{db::query::Query, entities::organizations};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_organizations(&self, ctx: &Context<'_>) -> Result<Vec<organizations::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        Query::get_all_organizations(db).await.map_err(Into::into)
    }

    async fn get_organization_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<organizations::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        Query::find_organization_by_id(db, id)
            .await
            .map_err(Into::into)
    }
}
