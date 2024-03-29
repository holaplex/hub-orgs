//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5
#![allow(clippy::all)]
use async_graphql::*;
use hub_core::{assets::AssetProxy, url::Url};
use sea_orm::{entity::prelude::*, Condition, QueryOrder};
use serde::{Deserialize, Serialize};

use super::{invites, members, owners, projects, sea_orm_active_enums::InviteStatus, Project};
use crate::AppContext;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub deactivated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub profile_image_url: Option<String>,
}

/// A Holaplex organization is the top-level account within the Holaplex ecosystem. Each organization has a single owner who can invite members to join. Organizations use projects to organize NFT campaigns or initiatives.
#[derive(Clone, SimpleObject, Debug)]
#[graphql(complex)]
pub struct Organization {
    /// The unique identifier assigned to the Holaplex organization, which is used to distinguish it from other organizations within the Holaplex ecosystem.
    pub id: Uuid,
    /// The name given to the Holaplex organization, which is used to identify it within the Holaplex ecosystem and to its members and users.
    pub name: String,
    /// The datetime, in UTC, when the Holaplex organization was created by its owner.
    pub created_at: DateTimeWithTimeZone,
    /// The datetime, in UTC, when the Holaplex organization was deactivated by its owner.
    pub deactivated_at: Option<DateTimeWithTimeZone>,
    /// The optional profile image associated with the Holaplex organization, which can be used to visually represent the organization.
    pub profile_image_url_original: Option<String>,
}

#[ComplexObject]
impl Organization {
    /// The members who have been granted access to the Holaplex organization, represented by individuals who have been invited and accepted the invitation to join the organization.
    async fn members(&self, ctx: &Context<'_>) -> Result<Option<Vec<members::Member>>> {
        let AppContext { members_loader, .. } = ctx.data::<AppContext>()?;
        members_loader.load_one(self.id).await
    }

    /// The owner of the Holaplex organization, who has created the organization and has full control over its settings and members.
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<owners::Owner>> {
        let AppContext { owner_loader, .. } = ctx.data::<AppContext>()?;
        owner_loader.load_one(self.id).await
    }

    /// The invitations to join the Holaplex organization that have been sent to email addresses and are either awaiting or have been accepted by the recipients.
    async fn invites(
        &self,
        ctx: &Context<'_>,
        status: Option<InviteStatus>,
    ) -> Result<Vec<invites::Model>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        let mut conditions = Condition::all();

        let query = invites::Entity::find();

        conditions = conditions.add(invites::Column::OrganizationId.eq(self.id));

        if let Some(status) = status {
            conditions = conditions.add(invites::Column::Status.eq(status));
        }

        query
            .filter(conditions)
            .order_by_desc(invites::Column::CreatedAt)
            .all(db.get())
            .await
            .map_err(Into::into)
    }

    /// The projects that have been created and are currently associated with the Holaplex organization, which are used to organize NFT campaigns or initiatives within the organization.
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        projects::Entity::find()
            .filter(projects::Column::OrganizationId.eq(self.id))
            .order_by_desc(projects::Column::CreatedAt)
            .all(db.get())
            .await
            .map_err(Into::into)
            .map(|projects| projects.into_iter().map(|p| p.into()).collect())
    }

    async fn profile_image_url(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        if let Some(image) = &self.profile_image_url_original {
            let asset_proxy = ctx.data::<AssetProxy>()?;
            let url = Url::parse(&image)?;
            asset_proxy
                .proxy_ipfs_image(&url, None)
                .map_err(|e| e.into())
                .map(|u| u.map(Into::into))
        } else {
            Ok(None)
        }
    }
}

impl From<Model> for Organization {
    fn from(
        Model {
            id,
            name,
            created_at,
            deactivated_at,
            profile_image_url,
        }: Model,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            deactivated_at,
            profile_image_url_original: profile_image_url,
        }
    }
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
