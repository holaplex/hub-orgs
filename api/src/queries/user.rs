use async_graphql::{Context, Object, Result, Union};
use sea_orm::{prelude::*, QueryOrder};

use crate::{
    entities::{members, owners},
    AppContext,
};

/// An enum type named Affiliation that defines a user's association to an organization. The enum is derived using a Union attribute. It has two variants, each containing an associated data type:
#[derive(Union)]
enum Affiliation {
    /// Owner variant contains a Owner data type, representing the owner of the organization.
    Owner(owners::Owner),
    /// Member variant contains a Member data type, representing a member of the organization.
    Member(members::Member),
}

/// a hub user
#[derive(Debug, Clone, Copy)]
pub struct User {
    pub id: Uuid,
}

impl From<owners::Model> for Affiliation {
    fn from(
        owners::Model {
            id,
            user_id,
            organization_id,
            created_at,
        }: owners::Model,
    ) -> Self {
        Self::Owner(owners::Owner {
            id,
            user_id,
            organization_id,
            created_at,
        })
    }
}

impl From<members::Model> for Affiliation {
    fn from(
        members::Model {
            id,
            user_id,
            organization_id,
            created_at,
            revoked_at,
            invite_id,
        }: members::Model,
    ) -> Self {
        Self::Member(members::Member {
            id,
            user_id,
            organization_id,
            created_at,
            revoked_at,
            invite_id,
        })
    }
}

#[Object]
impl User {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn affiliations(&self, ctx: &Context<'_>) -> Result<Vec<Affiliation>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let user_id = self.id;
        let conn = db.get();

        let org_owners = owners::Entity::find_by_user(user_id)
            .order_by_desc(owners::Column::CreatedAt)
            .all(conn)
            .await?;

        let org_members = members::Entity::find_by_user(user_id)
            .order_by_desc(members::Column::CreatedAt)
            .all(conn)
            .await?;

        Ok(org_owners
            .into_iter()
            .map(Into::into)
            .chain(org_members.into_iter().map(Into::into))
            .collect())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "UserQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_user_by_id(&self, #[graphql(key)] id: Uuid) -> Result<User> {
        let user = User { id };

        Ok(user)
    }
}
