use async_graphql::{Context, Error, Object, Result, SimpleObject, Value};
use hub_core::serde_json;
use sea_orm::prelude::*;
use svix::api::{EventTypeOut, Svix};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "WebhookQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn event_types(&self, ctx: &Context<'_>) -> Result<Vec<EventType>> {
        let svix = ctx.data::<Svix>()?;

        let event_types = svix.event_type().list(None).await?;

        event_types
            .data
            .iter()
            .map(|d| d.clone().try_into())
            .collect::<_>()
    }
}

#[derive(Clone, Debug, PartialEq, SimpleObject)]
#[graphql(concrete(name = "EventType", params()))]
pub struct EventType {
    pub archived: Option<bool>,
    pub created_at: String,
    pub description: String,
    pub name: String,
    pub schemas: Json,
    pub updated_at: String,
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
        let schema: Value = serde_json::to_string(&schemas)?.into();
        let json = schema.into_json()?;

        Ok(Self {
            archived,
            created_at,
            description,
            name,
            schemas: json,
            updated_at,
        })
    }
}
