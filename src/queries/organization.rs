use async_graphql::{self, Context, Object, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::organizations;
#[derive(Default)]
pub struct OrganizationQuery;

#[Object]
impl OrganizationQuery {
    async fn get_organizations(&self, ctx: &Context<'_>) -> Result<Vec<organizations::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        organizations::Entity::find()
            .all(db)
            .await
            .map_err(Into::into)
    }

    async fn get_organization_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<organizations::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;

        organizations::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Into::into)
    }
}
