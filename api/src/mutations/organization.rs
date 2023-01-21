use async_graphql::{self, Context, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};
use webhooks::api::ApplicationIn;

use crate::{
    db::DatabaseClient,
    entities::{organizations, organizations::ActiveModel, owners},
    svix_client::SvixClient,
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
        let svix = &**ctx.data::<SvixClient>()?;

        let user_id = id.ok_or_else(|| "no user id")?;

        let mut org_model = ActiveModel::from(input.clone()).insert(db).await?;

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
                org_model = org.update(db).await?;

                let owner = owners::ActiveModel {
                    user_id: Set(user_id),
                    organization_id: Set(org_model.id),
                    ..Default::default()
                };

                owner.insert(db).await?;
            },
            Err(err) => {
                org_model.delete(db).await?;
                return Err(err.into());
            },
        };

        Ok(org_model)
    }
}

#[derive(InputObject, Clone)]
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
