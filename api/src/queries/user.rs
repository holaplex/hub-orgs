use std::sync::Arc;

use async_graphql::{self, Context, Object, Result, Union};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use uuid::Uuid;

use crate::{
    entities::{members, owners},
    UserID,
};

#[derive(Union)]
enum Affiliation {
    Owner(owners::Model),
    Member(members::Model),
}

#[derive(Default)]
pub struct Query;

#[Object(name = "UserQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn affiliations(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 25)] limit: u64,
        #[graphql(default = 0)] offset: u64,
    ) -> Result<Vec<Affiliation>> {
        let UserID(id) = ctx.data::<UserID>()?;
        let user_id = Uuid::parse_str(id)?;

        let db = &**ctx.data::<Arc<DatabaseConnection>>()?;

        // No union all in sea query?

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
