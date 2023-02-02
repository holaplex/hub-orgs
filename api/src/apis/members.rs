use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{param::Header, payload::Json, OpenApi};
use sea_orm::prelude::*;

use crate::{entities::members, AppState};

pub struct Members;

#[OpenApi]
impl Members {
    #[oai(path = "/members", method = "get")]
    async fn get_members(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<Vec<members::Model>>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();

        let members = members::Entity::find()
            .filter(members::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(members))
    }
}
