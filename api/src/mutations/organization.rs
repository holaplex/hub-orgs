use async_graphql::{self, Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::producer::Producer;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{organizations, organizations::ActiveModel, owners},
    proto::{organization_events::Event, Organization, OrganizationEventKey, OrganizationEvents},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "OrganizationMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if unable to save organization to the database
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<CreateOrganizationPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let org_model = ActiveModel::from(input.clone()).insert(db.get()).await?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org_model.id),
            ..Default::default()
        };

        owner.insert(db.get()).await?;

        let event = OrganizationEvents {
            event: Some(Event::OrganizationCreated(org_model.clone().into())),
        };

        let key = OrganizationEventKey {
            id: org_model.id.to_string(),
            user_id: user_id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(CreateOrganizationPayload {
            organization: org_model.into(),
        })
    }
}

#[derive(Debug, InputObject, Clone)]
pub struct CreateOrganizationInput {
    pub name: String,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct CreateOrganizationPayload {
    pub organization: organizations::Organization,
}

impl From<CreateOrganizationInput> for ActiveModel {
    fn from(val: CreateOrganizationInput) -> Self {
        Self {
            name: Set(val.name),
            profile_image_url: Set(val.profile_image_url),
            ..Default::default()
        }
    }
}

impl From<organizations::Model> for Organization {
    fn from(
        organizations::Model {
            id,
            name,
            created_at,
            deactivated_at,
            ..
        }: organizations::Model,
    ) -> Self {
        Self {
            id: id.to_string(),
            name,
            created_at: created_at.to_string(),
            deactivated_at: deactivated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}
