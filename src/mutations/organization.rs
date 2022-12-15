use async_graphql::{self, Context, InputObject, Object, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entities::{organizations, organizations::ActiveModel};
#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn create_organization(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganizationInput,
    ) -> Result<organizations::Model> {
        let db = ctx.data::<DatabaseConnection>()?;

        ActiveModel::from(input)
            .insert(db)
            .await
            .map_err(Into::into)
    }
}

#[derive(InputObject)]
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
