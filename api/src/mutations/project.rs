use async_graphql::{self, Context, InputObject, Object, Result};
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
    ) -> Result<projects::Model> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        ActiveModel::from(input)
            .insert(db.get())
            .await
            .map_err(Into::into)
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
