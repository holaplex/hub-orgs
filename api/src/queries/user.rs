use async_graphql::{Context, Object, Result, Union};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};

use crate::{
    entities::{members, owners},
    AppContext,
};

#[derive(Union)]
enum Affiliation {
    Owner(owners::Owner),
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
        }: members::Model,
    ) -> Self {
        Self::Member(members::Member {
            id,
            user_id,
            organization_id,
            created_at,
            revoked_at,
        })
    }
}

#[Object]
impl User {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn affiliations(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 25)] limit: u64,
        #[graphql(default = 0)] offset: u64,
    ) -> Result<Vec<Affiliation>> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let user_id = self.id;
        let conn = db.get();

        let org_owners = owners::Entity::find()
            .filter(owners::Column::UserId.eq(user_id))
            .order_by_desc(owners::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(conn)
            .await?;

        let org_members = members::Entity::find()
            .filter(
                members::Column::UserId
                    .eq(user_id)
                    .and(members::Column::RevokedAt.is_null()),
            )
            .order_by_desc(owners::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
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
