//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use async_graphql::*;
use hub_core::{assets::AssetProxy, url::Url};
use sea_orm::entity::prelude::*;

use super::organizations::Organization;
use crate::AppContext;

/// A Holaplex project that belongs to an organization. Projects are used to group unique NFT campaigns or initiatives, and are used to assign objects that end customers will interact with, such as drops and wallets.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub organization_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(nullable)]
    pub deactivated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub profile_image_url: Option<String>,
}

/// A Holaplex project that belongs to an organization. Projects are used to group unique NFT campaigns or initiatives, and are used to assign objects that end customers will interact with, such as drops and wallets.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex, concrete(name = "Project", params()))]
pub struct Project {
    /// The unique identifier assigned to the Holaplex project.
    pub id: Uuid,
    /// The friendly name assigned to the Holaplex project to differentiate it from other projects belonging to the organization.
    pub name: String,
    /// The ID of the Holaplex organization to which the project belongs.
    pub organization_id: Uuid,
    /// The datetime, in UTC, when the project was created.
    pub created_at: DateTimeWithTimeZone,
    /// The date and time in Coordinated Universal Time (UTC) when the Holaplex project was created. Once a project is deactivated, objects that were assigned to the project can no longer be interacted with.
    pub deactivated_at: Option<DateTimeWithTimeZone>,
    /// The optional profile image associated with the project, which can be used to visually represent the project.
    pub profile_image_url_original: Option<String>,
}

#[ComplexObject]
impl Project {
    async fn organization(&self, ctx: &Context<'_>) -> Result<Option<Organization>> {
        let AppContext {
            organization_loader,
            ..
        } = ctx.data::<AppContext>()?;
        organization_loader.load_one(self.organization_id).await
    }

    async fn profile_image_url(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        if let Some(image) = &self.profile_image_url_original {
            let asset_proxy = ctx.data::<AssetProxy>()?;
            let url = Url::parse(image)?;
            asset_proxy
                .proxy_ipfs_image(&url, None)
                .map_err(|e| e.into())
                .map(|u| u.map(Into::into))
        } else {
            Ok(None)
        }
    }
}

impl From<Model> for Project {
    fn from(
        Model {
            id,
            name,
            organization_id,
            created_at,
            deactivated_at,
            profile_image_url,
        }: Model,
    ) -> Self {
        Self {
            id,
            name,
            organization_id,
            created_at,
            deactivated_at,
            profile_image_url_original: profile_image_url,
        }
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
