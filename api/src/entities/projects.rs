//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::*;
use sea_orm::entity::prelude::*;

use super::organizations::Organization;
use crate::AppContext;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "projects")]
#[graphql(complex, concrete(name = "Project", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub organization_id: Uuid,
    pub created_at: DateTime,
    #[sea_orm(nullable)]
    pub deactivated_at: Option<DateTime>,
}

#[ComplexObject]
impl Model {
    async fn organization(&self, ctx: &Context<'_>) -> Result<Option<Organization>> {
        let AppContext {
            organization_loader,
            ..
        } = ctx.data::<AppContext>()?;
        organization_loader.load_one(self.organization_id).await
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

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Self> {
        Self::find().filter(Column::Id.eq(id))
    }
}
