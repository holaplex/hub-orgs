use async_graphql::{dataloader::DataLoader, *};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::projects::Model as Project;
use crate::dataloaders::WebhookProjectsLoader;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "webhooks")]
#[graphql(complex, concrete(name = "Webhook", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub endpoint_id: String,
    pub organization_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub created_by: Uuid,
}
#[ComplexObject]
impl Model {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Option<Vec<Project>>> {
        let loader = ctx.data::<DataLoader<WebhookProjectsLoader>>()?;
        loader.load_one(self.id).await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Organizations,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
