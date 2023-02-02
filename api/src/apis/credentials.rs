use hub_core::uuid::Uuid;
use ory_openapi_generated_client::models::OAuth2Client;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{
    param::{Header, Path},
    payload::Json,
    Object, OpenApi,
};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{credentials, project_credentials},
    AppState,
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

pub struct Credentials;

#[OpenApi]
impl Credentials {
    #[oai(path = "/credentials", method = "get")]
    async fn list_credentials(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<Vec<credentials::Model>>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();

        let credentials = credentials::Entity::find()
            .filter(credentials::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(credentials))
    }

    #[oai(path = "/credentials", method = "post")]
    async fn create_credential(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        input: Json<CreateCredentialInput>,
    ) -> Result<Json<CreateCredentialPayload>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let Header(user) = user;
        let ory = &state.ory_client;
        let input = input.0;
        let conn = state.connection.get();

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
            created_by: Set(user),
            ..Default::default()
        };

        let credential_model = credential.insert(conn).await.map_err(InternalServerError)?;

        // insert project credentials
        for project in input.projects {
            let project_credentials = project_credentials::ActiveModel {
                credential_id: Set(credential_model.id),
                project_id: Set(project),
                created_by: Set(user),
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
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        credential: Path<Uuid>,
    ) -> Result<()> {
        let Data(state) = state;
        let Path(credential) = credential;
        let Header(organization) = organization;
        let ory = &state.ory_client;
        let conn = state.connection.get();

        let credential = credentials::Entity::find()
            .filter(
                credentials::Column::Id
                    .eq(credential)
                    .and(credentials::Column::OrganizationId.eq(organization)),
            )
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
