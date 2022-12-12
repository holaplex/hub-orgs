use hub_core::{
    async_graphql::{self, Context, Object, Result},
    db::{entities::organizations, mutation::Mutation, DatabaseConnection},
};

use super::objects;

#[derive(Default)]
pub struct OrganizationMutation;

#[Object]
impl OrganizationMutation {
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: objects::organization::CreateOrganizationInput,
    ) -> Result<organizations::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        // let conn = db.get();

        Mutation::create_organization(db, input.into_model())
            .await
            .map_err(Into::into)
    }
}
