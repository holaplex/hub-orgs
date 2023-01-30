use async_graphql::{self, Context, InputObject, Object, Result};
use hub_core::producer::Producer;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{projects, projects::ActiveModel},
    proto::{event::EventPayload, Event, Key, Project},
    AppContext,
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
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<Event>>()?;

        let model = ActiveModel::from(input).insert(db.get()).await?;

        let event = Event {
            event_payload: Some(EventPayload::ProjectCreated(model.clone().into())),
        };

        let key = Key {
            id: model.id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(model)
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
