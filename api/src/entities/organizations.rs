//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
#![allow(clippy::all)]
use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "organizations")]
#[graphql(concrete(name = "Organization", params()))]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: uuid::Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub created_at: DateTime,
    #[sea_orm(nullable)]
    pub deactivated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::members::Entity")]
    Members,
    #[sea_orm(has_one = "super::owners::Entity")]
    Owners,
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Self> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_name(name: &str) -> Select<Self> {
        Self::find().filter(Column::Name.eq(name))
    }
}
