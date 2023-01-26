use async_graphql::{self, Context, Error, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{organizations, organizations::ActiveModel, owners},
    AppContext, UserID,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "OrganizationMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<organizations::Model> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let UserID(id) = user_id;
        let conn = db.get();

        let user_id = id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let org = ActiveModel::from(input).insert(conn).await?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org.id),
            ..Default::default()
        };

        owner.insert(conn).await?;

        Ok(org)
    }
}

#[derive(Debug, InputObject)]
pub struct CreateOrganizationInput {
    pub name: String,
}

impl From<CreateOrganizationInput> for ActiveModel {
    fn from(val: CreateOrganizationInput) -> Self {
        Self {
            name: Set(val.name),
            ..Default::default()
        }
    }
}
