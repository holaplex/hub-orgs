use hub_core::uuid::Uuid;
use poem::{
    error::InternalServerError,
    http::StatusCode,
    web::{Data, Path},
    Error, Result,
};
use poem_openapi::{param::Header, payload::Json, Enum, Object, OpenApi};
use sea_orm::{prelude::*, Set};
use svix::api::{EndpointIn, EventTypeOut};

use crate::{
    entities::{organizations, webhook_projects, webhooks},
    AppState,
};

#[derive(Clone, Debug, PartialEq, Object)]
#[oai(read_only_all)]
pub struct EventType {
    pub archived: Option<bool>,
    pub created_at: String,
    pub description: String,
    pub name: String,
    pub schemas: serde_json::Value,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Enum)]
pub enum FilterType {
    ProjectCreated,
    ProjectDeactivated,
    InvitationSent,
    InvitationAccepted,
    InvitationRevoked,
    CredentialCreated,
    CredentialDeleted,
}

impl FilterType {
    fn format(self) -> String {
        match self {
            Self::ProjectCreated => "project.created".to_string(),
            Self::ProjectDeactivated => "project.deactivated".to_string(),
            Self::InvitationSent => "invitation.sent".to_string(),
            Self::InvitationAccepted => "invitation.accepted".to_string(),
            Self::InvitationRevoked => "invitation.revoked".to_string(),
            Self::CredentialCreated => "credential.created".to_string(),
            Self::CredentialDeleted => "credential.deleted".to_string(),
        }
    }
}

#[derive(Object, Clone)]
pub struct CreateWebhookInput {
    pub endpoint: String,
    pub organization: Uuid,
    pub projects: Vec<Uuid>,
    pub filter_types: Vec<FilterType>,
}

#[derive(Object, Debug, Clone)]
pub struct CreateWebhookPayload {
    pub webhook: webhooks::Model,
    pub secret: String,
}

pub struct Webhooks;

#[OpenApi]
impl Webhooks {
    #[oai(path = "/webhook/types", method = "get")]
    async fn get_event_types(&self, state: Data<&AppState>) -> Result<Json<Vec<EventType>>> {
        let Data(state) = state;
        let svix = &state.svix_client;

        let event_types = svix
            .event_type()
            .list(None)
            .await
            .map_err(InternalServerError)?;

        let event_types: Result<Vec<EventType>, _> = event_types
            .data
            .into_iter()
            .map(std::convert::TryInto::try_into)
            .collect();

        let event_types = event_types?;

        Ok(Json(event_types))
    }

    #[oai(path = "/webhooks", method = "post")]
    async fn create_webhook(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
        input: Json<CreateWebhookInput>,
    ) -> Result<Json<CreateWebhookPayload>> {
        let Data(state) = state;
        let svix = &state.svix_client;
        let conn = state.connection.get();
        let Header(user) = user;
        let Header(organization) = organization;
        let input = input.0;

        let org = organizations::Entity::find()
            .filter(organizations::Column::Id.eq(organization))
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let org = org.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        let app_id = org.svix_app_id;

        // create endpoint request body
        let create_endpoint = EndpointIn {
            channels: Some(input.projects.iter().map(ToString::to_string).collect()),
            filter_types: Some(input.filter_types.iter().map(|e| e.format()).collect()),
            version: 1,
            description: None,
            disabled: Some(false),
            rate_limit: None,
            secret: None,
            url: input.endpoint,
            uid: None,
        };

        // create endpoint
        let endpoint = svix
            .endpoint()
            .create(app_id.clone(), create_endpoint, None)
            .await
            .map_err(InternalServerError)?;

        // Ge the randomly generated endpoint secret
        let endpoint_secret = svix
            .endpoint()
            .get_secret(app_id, endpoint.clone().id)
            .await
            .map_err(InternalServerError)?;

        // insert the webhook record
        let webhook_active_model = webhooks::ActiveModel {
            endpoint_id: Set(endpoint.id),
            organization_id: Set(input.organization),
            updated_at: Set(None),
            created_by: Set(user),
            ..Default::default()
        };

        let webhook = webhook_active_model
            .insert(conn)
            .await
            .map_err(InternalServerError)?;

        // insert all the webhook projects
        for project in &input.projects {
            let webhook_project_active_model = webhook_projects::ActiveModel {
                webhook_id: Set(webhook.id),
                project_id: Set(*project),
                ..Default::default()
            };

            webhook_project_active_model
                .insert(conn)
                .await
                .map_err(InternalServerError)?;
        }

        Ok(Json(CreateWebhookPayload {
            webhook,
            secret: endpoint_secret.key,
        }))
    }

    #[oai(path = "/webhooks", method = "get")]
    async fn list_webhooks(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<Vec<webhooks::Model>>> {
        let Data(state) = state;
        let conn = state.connection.get();
        let Header(organization) = organization;

        let webhooks = webhooks::Entity::find()
            .filter(webhooks::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(webhooks))
    }

    #[oai(path = "/webhooks/{webhook}", method = "get")]
    async fn get_webhook(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        webhook: Path<Uuid>,
    ) -> Result<Json<webhooks::Model>> {
        let Data(state) = state;
        let conn = state.connection.get();
        let Header(organization) = organization;
        let Path(webhook) = webhook;

        let webhook = webhooks::Entity::find()
            .filter(
                webhooks::Column::OrganizationId
                    .eq(organization)
                    .and(webhooks::Column::Id.eq(webhook)),
            )
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let webhook = webhook.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        Ok(Json(webhook))
    }

    #[oai(path = "/webhooks/{webhook}", method = "delete")]
    async fn delete_webhook(
        &self,
        state: Data<&AppState>,
        webhook: Path<Uuid>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<()> {
        let Data(state) = state;
        let Path(webhook) = webhook;
        let svix = &state.svix_client;
        let conn = state.connection.get();
        let Header(organization) = organization;

        let org = organizations::Entity::find()
            .filter(organizations::Column::Id.eq(organization))
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let org = org.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        let webhook = webhooks::Entity::find()
            .filter(
                webhooks::Column::Id
                    .eq(webhook)
                    .and(webhooks::Column::OrganizationId.eq(organization)),
            )
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        let webhook = webhook.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;

        svix.endpoint()
            .delete(org.svix_app_id.clone(), webhook.endpoint_id.clone())
            .await
            .map_err(InternalServerError)?;

        webhook.delete(conn).await.map_err(InternalServerError)?;

        Ok(())
    }
}

impl TryFrom<EventTypeOut> for EventType {
    type Error = Error;

    fn try_from(
        EventTypeOut {
            archived,
            created_at,
            description,
            name,
            schemas,
            updated_at,
        }: EventTypeOut,
    ) -> Result<Self> {
        let schemas = serde_json::to_string(&schemas)
            .map_err(InternalServerError)?
            .into();

        Ok(Self {
            archived,
            created_at,
            description,
            name,
            schemas,
            updated_at,
        })
    }
}
