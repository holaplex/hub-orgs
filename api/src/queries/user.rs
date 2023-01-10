use std::sync::Arc;

use async_graphql::{Context, Object, Result, Union};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::entities::{members, owners};

#[derive(Union)]
enum Affiliation {
    Owner(owners::Model),
    Member(members::Model),
}

/// a hub user
pub struct User {
    pub id: Uuid,
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
        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;
        let user_id = self.id;

        let org_owners = owners::Entity::find()
            .filter(owners::Column::UserId.eq(user_id))
            .order_by_desc(owners::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        let org_members = members::Entity::find()
            .filter(members::Column::UserId.eq(user_id))
            .order_by_desc(members::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(org_owners
            .into_iter()
            .map(Into::into)
            .chain(org_members.into_iter().map(Into::into))
            .collect())
    }
}

#[derive(Default)]
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
