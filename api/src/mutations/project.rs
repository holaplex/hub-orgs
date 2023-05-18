use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::producer::Producer;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{
        projects,
        projects::{ActiveModel, Project},
    },
    proto::{
        organization_events::Event, OrganizationEventKey, OrganizationEvents,
        Project as ProtoProject,
    },
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "ProjectMutation")]
impl Mutation {
    /// This mutation creates a new project under the specified organization.
    ///
    /// # Errors
    /// This mutation produces an error if it is unable to connect to the database, emit the project creation event, or if the user is not set in the X-USER-ID header.
    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<CreateProjectPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        let id = user_id.ok_or_else(|| "X-USER-ID header not found")?;

        let project: Project = ActiveModel::from(input).insert(db.get()).await?.into();

        let event = OrganizationEvents {
            event: Some(Event::ProjectCreated(project.clone().into())),
        };

        let key = OrganizationEventKey {
            id: project.id.to_string(),
            user_id: id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(CreateProjectPayload { project })
    }

    /// This mutations edits the name and profile image of the project.
    pub async fn edit_project(
        &self,
        ctx: &Context<'_>,
        input: EditProjectInput,
    ) -> Result<EditProjectPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let conn = db.get();

        let project = projects::Entity::find_by_id(input.id)
            .one(conn)
            .await?
            .ok_or_else(|| Error::new("project not found"))?;

        let mut active_project: projects::ActiveModel = project.into();

        active_project.name = Set(input.name);
        active_project.profile_image_url = Set(input.profile_image_url);

        let project: Project = active_project.update(conn).await?.into();

        Ok(EditProjectPayload { project })
    }
}

/// The input used for creating a project.
#[derive(Debug, InputObject)]
pub struct CreateProjectInput {
    /// The ID of the organization the project belongs to.
    pub organization: Uuid,
    /// The friendly name to denote the project from others belonging to the organization.
    pub name: String,
    /// The URL of the project's profile image.
    pub profile_image_url: Option<String>,
}

/**
 * The payload returned by the `createProject` mutation.
 */
#[derive(Debug, SimpleObject)]
pub struct CreateProjectPayload {
    /**
     * The project that was created.
     */
    pub project: Project,
}

impl From<CreateProjectInput> for ActiveModel {
    fn from(val: CreateProjectInput) -> Self {
        Self {
            organization_id: Set(val.organization),
            name: Set(val.name),
            profile_image_url: Set(val.profile_image_url),
            ..Default::default()
        }
    }
}

impl From<Project> for ProtoProject {
    fn from(
        Project {
            id,
            name,
            organization_id,
            created_at,
            deactivated_at,
            ..
        }: Project,
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

#[derive(Debug, InputObject)]
pub struct EditProjectInput {
    pub id: Uuid,
    pub name: String,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, SimpleObject)]
pub struct EditProjectPayload {
    pub project: Project,
}
