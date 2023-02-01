use hub_core::uuid::Uuid;
use ory_openapi_generated_client::models::OAuth2Client;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};
use sea_orm::{prelude::*, QueryOrder, Set};

use crate::{
    entities::{
        credentials, invites, members, organizations, owners, project_credentials, projects,
        sea_orm_active_enums,
    },
    AppState, UserID,
};

// Request payload for creating a credential
#[derive(Debug, Clone, Object)]
pub struct CreateCredentialPayload {
    credential: credentials::Model,
    client_secret: String,
    registration_access_token: Option<String>,
    registration_client_uri: Option<String>,
}

// Request input for creating a credential
#[derive(Object, Clone, Debug)]
pub struct CreateCredentialInput {
    pub organization: Uuid,
    pub name: String,
    pub projects: Vec<Uuid>,
}

#[derive(Debug, PartialEq, poem_openapi::Union)]
#[oai(discriminator_name = "type")]
enum Affiliation {
    Owner(owners::Model),
    Member(members::Model),
}

pub struct OrgsApi;

#[OpenApi]
impl OrgsApi {
    #[oai(path = "/affiliations", method = "get")]
    async fn list_user_affiliations(
        &self,
        state: Data<&AppState>,
        user_id: UserID,
    ) -> Result<Json<Vec<Affiliation>>> {
        let UserID(id) = user_id;
        let Data(state) = state;
        let conn = state.connection.get();

        let user_id = id.ok_or_else(|| Error::from_status(StatusCode::BAD_REQUEST))?;

        let org_owners = owners::Entity::find()
            .filter(owners::Column::UserId.eq(user_id))
            .order_by_desc(owners::Column::CreatedAt)
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        let org_members = members::Entity::find()
            .filter(
                members::Column::UserId
                    .eq(user_id)
                    .and(members::Column::RevokedAt.is_null()),
            )
            .order_by_desc(owners::Column::CreatedAt)
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(
            org_owners
                .into_iter()
                .map(|owner| Affiliation::Owner(owner))
                .chain(
                    org_members
                        .into_iter()
                        .map(|member| Affiliation::Member(member)),
                )
                .collect(),
        ))
    }

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
        _user_id: UserID,
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

    #[oai(path = "/organizations/{organization}/invites", method = "get")]
    async fn get_invites(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        organization: Path<Uuid>,
    ) -> Result<Json<Vec<invites::Model>>> {
        let Data(state) = state;
        let Path(organization) = organization;
        let conn = state.connection.get();

        let invites = invites::Entity::find()
            .filter(invites::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(invites))
    }

    #[oai(path = "/organizations/{organization}/invites", method = "post")]
    async fn create_invite(
        &self,
        state: Data<&AppState>,
        user_id: UserID,
        organization: Path<Uuid>,
        invite: Json<invites::Model>,
    ) -> Result<Json<invites::Model>> {
        let Data(state) = state;
        let Path(organization) = organization;
        let UserID(id) = user_id;
        let invite = invite.0;

        let user_id = id.ok_or_else(|| Error::from_status(StatusCode::BAD_REQUEST))?;
        let conn = state.connection.get();

        let active_model = invites::ActiveModel {
            organization_id: Set(organization),
            email: Set(invite.email),
            status: Set(sea_orm_active_enums::InviteStatus::Sent),
            created_by: Set(user_id),
            ..Default::default()
        };

        let invite = active_model
            .insert(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(invite))
    }

    #[oai(path = "/invites/{invite}/accept", method = "put")]
    async fn accept_invite(
        &self,
        state: Data<&AppState>,
        user_id: UserID,
        invite: Path<Uuid>,
    ) -> Result<Json<invites::Model>> {
        let Data(state) = state;
        let Path(id) = invite;
        let UserID(id) = user_id;
        let invite = invite.0;

        let user_id = id.ok_or_else(|| Error::from_status(StatusCode::BAD_REQUEST))?;
        let conn = state.connection.get();

        let invite = invites::Entity::find()
            .filter(invites::Column::Id.eq(id))
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        // TODO: check that the email of the logged in user matches the invite

        let invite = invite.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;
        let mut invite: invites::ActiveModel = invite.into();

        invite.status = Set(sea_orm_active_enums::InviteStatus::Accepted);

        let invite = invite.insert(conn).await.map_err(InternalServerError)?;

        Ok(Json(invite))
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

    #[oai(path = "/projects", method = "get")]
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

    #[oai(path = "/organizations/{organization}/credentials", method = "get")]
    async fn list_credentials(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        organization: Path<Uuid>,
    ) -> Result<Json<Vec<credentials::Model>>> {
        let Data(state) = state;
        let Path(organization) = organization;
        let conn = state.connection.get();

        let credentials = credentials::Entity::find()
            .filter(credentials::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(credentials))
    }

    #[oai(path = "/organizations/{organization}/credentials", method = "post")]
    async fn create_credential(
        &self,
        state: Data<&AppState>,
        user_id: UserID,
        organization: Path<Uuid>,
        input: Json<CreateCredentialInput>,
    ) -> Result<Json<CreateCredentialPayload>> {
        let Data(state) = state;
        let Path(organization) = organization;
        let UserID(id) = user_id;
        let ory = &state.ory_client;
        let input = input.0;
        let conn = state.connection.get();

        let user_id = id.ok_or_else(|| Error::from_status(StatusCode::BAD_REQUEST))?;

        // ory client post request payload
        let request_payload = OAuth2Client {
            grant_types: Some(vec!["client_credentials".to_string()]),
            client_name: Some(input.name.clone()),
            client_secret: None,
            owner: Some(organization.to_string()),
            client_credentials_grant_access_token_lifespan: None,
            ..Default::default()
        };

        // create oauth_2 using ory client
        let ory_response: OAuth2Client = ory.post("/clients", request_payload).await?;

        let client_id = ory_response
            .client_id
            .ok_or_else(|| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        let client_secret = ory_response
            .client_secret
            .ok_or_else(|| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

        // insert credential
        let credential = credentials::ActiveModel {
            name: Set(input.name),
            organization_id: Set(organization),
            client_id: Set(client_id),
            created_by: Set(user_id),
            ..Default::default()
        };

        let credential_model = credential.insert(conn).await.map_err(InternalServerError)?;

        // insert project credentials
        for project in input.projects {
            let project_credentials = project_credentials::ActiveModel {
                credential_id: Set(credential_model.id),
                project_id: Set(project),
                created_by: Set(user_id),
                ..Default::default()
            };

            project_credentials
                .insert(conn)
                .await
                .map_err(InternalServerError)?;
        }

        //  response
        let payload = CreateCredentialPayload {
            credential: credential_model.clone(),
            client_secret,
            registration_access_token: ory_response.registration_access_token,
            registration_client_uri: ory_response.registration_client_uri,
        };

        Ok(Json(payload))
    }

    #[oai(path = "/credentials/{credential}", method = "delete")]
    async fn delete_credential(
        &self,
        state: Data<&AppState>,
        _user_id: UserID,
        credential: Path<Uuid>,
    ) -> Result<()> {
        let Data(state) = state;
        let Path(credential) = credential;
        let ory = &state.ory_client;
        let conn = state.connection.get();

        let credential = credentials::Entity::find_by_id(credential)
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let credential = credential.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        let client_id = credential.client_id.clone();

        let endpoint = format!("/clients/{client_id}");

        let res = ory.delete(&endpoint).await?;

        if res.status() != StatusCode::NO_CONTENT {
            res.text().await.map_err(InternalServerError)?;
        }

        credential.delete(conn).await.map_err(InternalServerError)?;

        Ok(())
    }
}
