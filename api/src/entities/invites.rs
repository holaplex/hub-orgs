//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{members, sea_orm_active_enums::InviteStatus};
use crate::AppContext;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Serialize, Deserialize)]
#[sea_orm(table_name = "invites")]
#[graphql(complex, concrete(name = "Invite", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email: String,
    pub status: InviteStatus,
    pub organization_id: Uuid,
    pub created_by: Uuid,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[ComplexObject]
impl Model {
    async fn member(&self, ctx: &Context<'_>) -> Result<Option<members::Member>> {
        let AppContext {
            invite_member_loader,
            ..
        } = ctx.data::<AppContext>()?;

        invite_member_loader.load_one(self.id).await
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
