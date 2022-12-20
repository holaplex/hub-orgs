use async_graphql::{self, Context, InputObject, Object, Result};
use sea_orm::{prelude::*, Set};
use uuid::Uuid;

use crate::entities::{projects, projects::ActiveModel};

#[derive(Default)]
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
        let db = ctx.data_unchecked::<DatabaseConnection>();

        ActiveModel::from(input)
            .insert(db)
            .await
            .map_err(Into::into)
    }
}

#[derive(InputObject)]
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
