use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{entities::projects::Model, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "WebhookQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_webhook_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
        channels: Option<Vec<String>>,
    ) -> Result<Webhook> {
        Ok(Webhook {
            id,
            channels: channels.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, SimpleObject, Default)]
#[graphql(complex)]
pub struct Webhook {
    pub id: Uuid,
    #[graphql(external)]
    pub channels: Vec<String>,
}

#[ComplexObject]
impl Webhook {
    #[graphql(requires = "channels")]
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Model>> {
        let AppContext { project_loader, .. } = ctx.data::<AppContext>()?;

        let project_ids = self
            .channels
            .iter()
            .map(|channel| Uuid::parse_str(channel))
            .collect::<Result<Vec<Uuid>, _>>()?;

        let projects = project_loader
            .load_many(project_ids)
            .await?
            .into_values()
            .collect();

        Ok(projects)
    }
}
