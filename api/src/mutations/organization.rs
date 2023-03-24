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
    /// This mutation creates a new Holaplex organization, with the user triggering the mutation automatically assigned as the owner of the organization.
    /// # Errors
    /// This mutation produces an error if it is unable to connect to the database, emit the organization creation event, or if the user is not set in the X-USER-ID header.
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<CreateOrganizationPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let conn = db.get();
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let org_model = ActiveModel::from(input.clone()).insert(conn).await?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org_model.id),
            ..Default::default()
        };

        owner.insert(conn).await?;

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

    /// This mutation edits the name or profile image of the organization.
    pub async fn edit_organization(
        &self,
        ctx: &Context<'_>,
        input: EditOrganizationInput,
    ) -> Result<EditOrganizationPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let conn = db.get();

        let org = organizations::Entity::find_by_id(input.id)
            .one(conn)
            .await?
            .ok_or_else(|| Error::new("organization not found"))?;

        let mut active_org: organizations::ActiveModel = org.into();

        active_org.name = Set(input.name);
        active_org.profile_image_url = Set(input.profile_image_url);

        let org = active_org.update(conn).await?;

        Ok(EditOrganizationPayload {
            organization: org.into(),
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

#[derive(Debug, InputObject, Clone)]
pub struct EditOrganizationInput {
    pub id: Uuid,
    pub name: String,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct EditOrganizationPayload {
    pub organization: organizations::Organization,
}
