use async_graphql::{self, Context, Error, InputObject, Object, Result};
use hub_core::producer::Producer;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{projects, projects::ActiveModel},
    proto::{organization_events::Event, OrganizationEventKey, OrganizationEvents, Project},
    AppContext, UserID,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "ProjectMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<projects::Model> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;
        let UserID(user_id) = user_id;
        let id = user_id.ok_or_else(|| "X-USER-ID header not found")?;

        let project = ActiveModel::from(input)
            .insert(db.get())
            .await
            .map_err(Into::<Error>::into)?;

        let event = OrganizationEvents {
            event: Some(Event::ProjectCreated(project.clone().into())),
        };

        let key = OrganizationEventKey {
            id: project.id.to_string(),
            user_id: id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(project)
    }
}

#[derive(Debug, InputObject)]
pub struct CreateProjectInput {
    pub organization: Uuid,
    pub name: String,
}

impl From<CreateProjectInput> for ActiveModel {
    fn from(val: CreateProjectInput) -> Self {
        Self {
            organization_id: Set(val.organization),
            name: Set(val.name),
            ..Default::default()
        }
    }
}

impl From<projects::Model> for Project {
    fn from(
        projects::Model {
            id,
            name,
            organization_id,
            created_at,
            deactivated_at,
        }: projects::Model,
    ) -> Self {
        Self {
            id: id.to_string(),
            name,
            organization_id: organization_id.to_string(),
            created_at: created_at.to_string(),
            deactivated_at: deactivated_at.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}
