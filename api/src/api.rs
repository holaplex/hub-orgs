use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{payload::Json, OpenApi};
use sea_orm::{prelude::*, QueryOrder, Set};

use crate::{
    entities::{organizations, owners},
    AppState, UserID,
};

pub struct OrgsApi;

#[OpenApi]
impl OrgsApi {
    #[oai(path = "/organizations", method = "post")]
    async fn create_organization(
        &self,
        state: Data<&AppState>,
        user_id: UserID,
        organization: Json<organizations::Model>,
    ) -> Result<Json<organizations::Model>> {
        let UserID(id) = user_id;
        let Data(state) = state;
        let conn = state.connection.get();

        let user_id = id.ok_or_else(|| Error::from_status(StatusCode::BAD_REQUEST))?;

        let org = organizations::ActiveModel::from(organization.0)
            .insert(conn)
            .await
            .map_err(InternalServerError)?;

        let owner = owners::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(org.id),
            ..Default::default()
        };

        owner.insert(conn).await.map_err(InternalServerError)?;

        Ok(Json(org))
    }

    #[oai(path = "/organizations", method = "get")]
    async fn list_organizations(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
    ) -> Result<Json<Vec<organizations::Model>>> {
        let Data(state) = state;
        let conn = state.connection.get();

        let organizations = organizations::Entity::find()
            .order_by_desc(organizations::Column::CreatedAt)
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(organizations))
    }
}
