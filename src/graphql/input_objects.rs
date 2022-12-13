use async_graphql::{self, InputObject};
use sea_orm::Set;

use crate::{entities::organizations, prelude::*};

#[derive(InputObject)]
pub struct CreateOrganizationInput {
    pub name: String,
}

impl CreateOrganizationInput {
    pub fn into_active_model(self) -> organizations::ActiveModel {
        organizations::ActiveModel {
            name: Set(self.name),
            ..Default::default()
        }
    }
}
