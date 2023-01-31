use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sea_orm::{prelude::*, QueryOrder, Set};

use crate::{
    entities::{members, organizations, owners, projects},
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

    #[oai(path = "/organizations/{organization}", method = "get")]
    async fn get_organization(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        id: Path<Uuid>,
    ) -> Result<Json<organizations::Model>> {
        let Data(state) = state;
        let Path(id) = id;
        let conn = state.connection.get();

        let organization = organizations::Entity::find_by_id(id)
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let organization = organization.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        Ok(Json(organization))
    }

    #[oai(path = "/organizations/{organization}/members", method = "get")]
    async fn get_members(
        &self,
        state: Data<&AppState>,
        _use_id: UserID,
        organization: Path<Uuid>,
    ) -> Result<Json<Vec<members::Model>>> {
        let Data(state) = state;
        let Path(organization) = organization;
        let conn = state.connection.get();

        let members = members::Entity::find()
            .filter(members::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(members))
    }

    #[oai(path = "/organizations/{organization}/projects", method = "post")]
    async fn create_project(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        organization: Path<Uuid>,
        project: Json<projects::Model>,
    ) -> Result<Json<projects::Model>> {
        let Data(state) = state;
        let Path(organization) = organization;
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

    #[oai(path = "/organizations/{organization}/projects", method = "get")]
    async fn list_projects(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        organization: Path<Uuid>,
    ) -> Result<Json<Vec<projects::Model>>> {
        let Data(state) = state;
        let Path(organization) = organization;
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
        _user_id: UserID,
        id: Path<Uuid>,
    ) -> Result<Json<projects::Model>> {
        let Data(state) = state;
        let Path(id) = id;
        let conn = state.connection.get();

        let project = projects::Entity::find()
            .filter(projects::Column::Id.eq(id))
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let project = project.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        Ok(Json(project))
    }
}
