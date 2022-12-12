use async_graphql::{self, Context, Object, Result};
use sea_orm::DatabaseConnection;

use super::input_objects::CreateOrganizationInput;
use crate::{db::mutation::Mutation, entities::organizations};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<organizations::Model> {
        let db = ctx.data::<DatabaseConnection>()?;
        // let conn = db.get();

        Mutation::create_organization(db, input.into_model())
            .await
            .map_err(Into::into)
    }
}
