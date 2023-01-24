use std::sync::Arc;

use async_graphql::{self, Context, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{organizations, organizations::ActiveModel, owners},
    UserID,
};

#[derive(Debug, Default)]
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
        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;

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
