use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{
    param::{Header, Path},
    payload::Json,
    OpenApi,
};
use sea_orm::{prelude::*, QueryOrder, Set};

use crate::{entities::projects, AppState};

pub struct Projects;

#[OpenApi]
impl Projects {
    #[oai(path = "/projects", method = "post")]
    async fn create_project(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        project: Json<projects::Model>,
    ) -> Result<Json<projects::Model>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();
        let model = project.0;

        let project = projects::ActiveModel {
            organization_id: Set(organization),
            name: Set(model.name),
            ..Default::default()
        };

        let project = project.insert(conn).await.map_err(InternalServerError)?;

        Ok(Json(project))
    }

    #[oai(path = "/projects", method = "get")]
    async fn list_projects(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<Vec<projects::Model>>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();

        let projects = projects::Entity::find()
            .order_by_desc(projects::Column::CreatedAt)
            .filter(projects::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(projects))
    }

    #[oai(path = "/projects/{project}", method = "get")]
    async fn get_project(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        id: Path<Uuid>,
    ) -> Result<Json<projects::Model>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let Path(id) = id;
        let conn = state.connection.get();

        let project = projects::Entity::find()
            .filter(
                projects::Column::Id
                    .eq(id)
                    .and(projects::Column::OrganizationId.eq(organization)),
            )
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let project = project.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        Ok(Json(project))
    }
}
