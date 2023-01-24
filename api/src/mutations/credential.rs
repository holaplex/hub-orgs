use std::sync::Arc;

use async_graphql::{self, Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::reqwest::StatusCode;
use ory_openapi_generated_client::models::OAuth2Client;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{credentials, project_credentials},
    ory_client::OryClient,
    UserID,
};

#[derive(Debug, Default)]
pub struct Mutation;

#[Object(name = "CredentialMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_credential(
        &self,
        ctx: &Context<'_>,
        input: CreateCredentialInput,
    ) -> Result<CreateCredentialPayload> {
        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;
        let ory = ctx.data::<OryClient>()?;
        let UserID(id) = ctx.data::<UserID>()?;

        let user_id = id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        // ory client post request payload
        let request_payload = OAuth2Client {
            grant_types: Some(vec!["client_credentials".to_string()]),
            client_name: Some(input.name.clone()),
            client_secret: None,
            owner: Some(input.organization.to_string()),
            client_credentials_grant_access_token_lifespan: None,
            ..Default::default()
        };

        // create oauth_2 using ory client
        let ory_response: OAuth2Client = ory.post("/clients", request_payload).await?;

        let client_id = ory_response
            .client_id
            .ok_or_else(|| Error::new("Invalid response! client_id is null"))?;
        let client_secret = ory_response
            .client_secret
            .ok_or_else(|| Error::new("Invalid response! client secret is null"))?;

        // insert credential
        let credential = credentials::ActiveModel {
            name: Set(input.name),
            organization_id: Set(input.organization),
            client_id: Set(client_id),
            created_by: Set(user_id),
            ..Default::default()
        };

        let credential_model = credential.insert(db).await?;

        // insert project credentials
        for project in input.projects {
            let project_credentials = project_credentials::ActiveModel {
                credential_id: Set(credential_model.id),
                project_id: Set(project),
                created_by: Set(user_id),
                ..Default::default()
            };

            project_credentials.insert(db).await?;
        }

        // graphql response
        let graphql_response = CreateCredentialPayload {
            credential: credential_model.clone(),
            client_secret,
            registration_access_token: ory_response.registration_access_token,
            registration_client_uri: ory_response.registration_client_uri,
        };

        Ok(graphql_response)
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn delete_credential(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<DeleteCredentialPayload> {
        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;
        let ory = ctx.data::<OryClient>()?;

        let credential = credentials::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::new("Credential not found in db"))?;

        let client_id = credential.client_id.clone();

        let endpoint = format!("/clients/{client_id}");

        let res = ory.delete(&endpoint).await?;

        if res.status() != StatusCode::NO_CONTENT {
            let response_text = res.text().await?;
            return Err(Error::new(response_text));
        }

        credential.delete(db).await?;

        Ok(DeleteCredentialPayload { credential: id })
    }
}

#[derive(InputObject, Clone)]
pub struct CreateCredentialInput {
    pub organization: Uuid,
    pub name: String,
    pub projects: Vec<Uuid>,
}

// Request payload for creating a credential
#[derive(Debug, Clone, SimpleObject)]
pub struct CreateCredentialPayload {
    credential: credentials::Model,
    client_secret: String,
    registration_access_token: Option<String>,
    registration_client_uri: Option<String>,
}

// Request payload for deleting a credential
#[derive(Debug, Clone, SimpleObject)]
pub struct DeleteCredentialPayload {
    credential: Uuid,
}
