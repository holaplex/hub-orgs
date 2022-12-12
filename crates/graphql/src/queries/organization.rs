use hub_core::{
    async_graphql,
    async_graphql::{Context, Object, Result},
    db::{entities::organizations, query::Query, DatabaseConnection},
};

#[derive(Default)]
pub struct OrganizationQuery;

#[Object]
impl OrganizationQuery {
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
