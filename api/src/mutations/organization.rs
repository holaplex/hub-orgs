use async_graphql::{self, Context, Error, InputObject, Object, Result, SimpleObject};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{organizations, organizations::ActiveModel, owners},
    AppContext,
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
    ) -> Result<CreateOrganizationPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let org_model = ActiveModel::from(input.clone()).insert(db.get()).await?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org_model.id),
            ..Default::default()
        };

        owner.insert(db.get()).await?;

        Ok(CreateOrganizationPayload {
            organization: org_model.into(),
        })
    }
}

#[derive(Debug, InputObject, Clone)]
pub struct CreateOrganizationInput {
    pub name: String,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct CreateOrganizationPayload {
    pub organization: organizations::Organization,
}

impl From<CreateOrganizationInput> for ActiveModel {
    fn from(val: CreateOrganizationInput) -> Self {
        Self {
            name: Set(val.name),
            ..Default::default()
        }
    }
}
