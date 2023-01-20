use async_graphql::{self, Context, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};

use crate::{
    db::DatabaseClient,
    entities::{organizations, organizations::ActiveModel, owners},
    UserID,
};

#[derive(Default)]
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
        let UserID(id) = ctx.data::<UserID>()?;
        let db = &**ctx.data::<DatabaseClient>()?;

        let user_id = id.ok_or_else(|| "no user id")?;

        let org = ActiveModel::from(input).insert(db).await?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org.id),
            ..Default::default()
        };

        owner.insert(db).await?;

        Ok(org)
    }
}

#[derive(InputObject)]
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
