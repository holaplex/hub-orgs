use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{param::Header, payload::Json, OpenApi};
use sea_orm::{prelude::*, Set};
use svix::api::ApplicationIn;

use crate::{
    entities::{organizations, organizations::ActiveModel, owners},
    AppState,
};

pub struct Organizations;

#[OpenApi]
impl Organizations {
    #[oai(path = "/organization", method = "get")]
    async fn get_organization(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<organizations::Model>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();

        let organization = organizations::Entity::find_by_id(organization)
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let organization = organization.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        Ok(Json(organization))
    }

    #[oai(path = "/organizations", method = "post")]
    async fn create_organization(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
        input: Json<organizations::Model>,
    ) -> Result<Json<organizations::Model>> {
        let Header(user) = user;
        let Data(state) = state;
        let svix = &state.svix_client;
        let conn = state.connection.get();
        let input = input.0;

        let org_model = organizations::ActiveModel::from(input.clone())
            .insert(conn)
            .await
            .map_err(InternalServerError)?;

        let owner = owners::ActiveModel {
            user_id: Set(user),
            organization_id: Set(org_model.id),
            ..Default::default()
        };

        owner.insert(conn).await.map_err(InternalServerError)?;

        match svix
            .application()
            .create(
                ApplicationIn {
                    name: input.name.clone(),
                    rate_limit: None,
                    uid: Some(org_model.id.to_string()),
                },
                None,
            )
            .await
        {
            Ok(res) => {
                let mut org: ActiveModel = org_model.clone().into();
                org.svix_app_id = Set(res.id);

                let updated_org_model = org.update(conn).await.map_err(InternalServerError)?;

                Ok(Json(updated_org_model))
            },
            Err(err) => {
                org_model.delete(conn).await.map_err(InternalServerError)?;

                Err(InternalServerError(err))
            },
        }
    }
}
