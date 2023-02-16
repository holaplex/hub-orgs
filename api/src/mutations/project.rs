use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{projects, projects::ActiveModel},
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
    ) -> Result<CreateProjectPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        let project = ActiveModel::from(input).insert(db.get()).await?;

        Ok(CreateProjectPayload { project })
    }
}

#[derive(Debug, InputObject)]
pub struct CreateProjectInput {
    pub organization: Uuid,
    pub name: String,
}

#[derive(Debug, SimpleObject)]
pub struct CreateProjectPayload {
    pub project: projects::Model,
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
