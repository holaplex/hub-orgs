use async_graphql::{self, InputObject};

use crate::{entities::organizations, prelude::*};

#[derive(InputObject)]
pub struct CreateOrganizationInput {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub deactivated_at: DateTime<Utc>,
}

impl CreateOrganizationInput {
    pub fn into_model(self) -> organizations::Model {
        organizations::Model {
            id: Default::default(),
            name: self.name,
            created_at: self.created_at.naive_utc(),
            deactivated_at: self.deactivated_at.naive_utc(),
        }
    }
}
