//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::{dataloader::DataLoader, *};
use sea_orm::entity::prelude::*;

use super::projects::Model as Project;
use crate::ProjectLoader;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "project_credentials")]
#[graphql(complex, concrete(name = "ProjectCredential", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub credential_id: Uuid,
    pub project_id: Uuid,
    pub created_at: DateTime,
    pub created_by: Uuid,
}

#[ComplexObject]
impl Model {
    async fn project(&self, ctx: &Context<'_>) -> Result<Option<Project>> {
        let loader = ctx.data::<DataLoader<ProjectLoader>>()?;
        loader.load_one(self.project_id).await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::credentials::Entity",
        from = "Column::CredentialId",
        to = "super::credentials::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Credentials,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Projects,
}

impl Related<super::credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Credentials.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}