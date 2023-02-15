use async_graphql::{self, Context, Error, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};
use svix::api::{ApplicationIn, Svix};

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
    ) -> Result<organizations::Organization> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;

        let svix = ctx.data::<Svix>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let mut org_model = ActiveModel::from(input.clone()).insert(db.get()).await?;

        match svix
            .application()
            .create(
                ApplicationIn {
                    name: input.name,
                    rate_limit: None,
                    uid: Some(org_model.id.to_string()),
                },
                None,
            )
            .await
        {
            Ok(res) => {
                let mut org: ActiveModel = org_model.clone().into();
                org.svix_app_id = Set(res.id);
                org_model = org.update(db.get()).await?;

                let owner = owners::ActiveModel {
                    user_id: Set(user_id),
                    organization_id: Set(org_model.id),
                    ..Default::default()
                };

                owner.insert(db.get()).await?;
            },
            Err(err) => {
                org_model.delete(db.get()).await?;
                return Err(err.into());
            },
        };

        Ok(org_model.into())
    }
}

#[derive(Debug, InputObject, Clone)]
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
