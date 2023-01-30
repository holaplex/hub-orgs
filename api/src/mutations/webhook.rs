use async_graphql::{self, Context, Enum, Error, InputObject, Object, Result, SimpleObject};
use sea_orm::{prelude::*, Set};
use svix::api::{EndpointIn, Svix};

use crate::{
    entities::{
        organizations,
        webhook_projects::ActiveModel as WebhookProjectActiveModel,
        webhooks::{ActiveModel as WebhookActiveModel, Model as Webhook},
    },
    AppContext, UserID,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "WebhookMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_webhook(
        &self,
        ctx: &Context<'_>,
        input: CreateWebhookInput,
    ) -> Result<CreateWebhookPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let UserID(id) = user_id;
        let svix = ctx.data::<Svix>()?;

        let user_id = id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        // Find organization from database to get the svix app id
        if let Some(org) = organizations::Entity::find_by_id(input.organization)
            .one(db.get())
            .await?
        {
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
                .await?;

            // Ge the randomly generated endpoint secret
            let endpoint_secret = svix
                .endpoint()
                .get_secret(app_id, endpoint.clone().id)
                .await?;

            // insert the webhook record
            let webhook_active_model = WebhookActiveModel {
                endpoint_id: Set(endpoint.id),
                organization_id: Set(input.organization),
                updated_at: Set(None),
                created_by: Set(user_id),
                ..Default::default()
            };

            let webhook = webhook_active_model.insert(db.get()).await?;

            // insert all the webhook projects
            for project in &input.projects {
                let webhook_project_active_model = WebhookProjectActiveModel {
                    webhook_id: Set(webhook.id),
                    project_id: Set(*project),
                    ..Default::default()
                };

                webhook_project_active_model.insert(db.get()).await?;
            }

            // return the webhook object and endpoint secret
            let graphql_response = CreateWebhookPayload {
                webhook,
                secret: endpoint_secret.key,
            };

            Ok(graphql_response)
        } else {
            Err(Error::new("Organization not found in database"))
        }
    }
}

#[derive(InputObject, Clone)]
pub struct CreateWebhookInput {
    pub endpoint: String,
    pub organization: Uuid,
    pub projects: Vec<Uuid>,
    pub filter_types: Vec<FilterType>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct CreateWebhookPayload {
    pub webhook: Webhook,
    pub secret: String,
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
