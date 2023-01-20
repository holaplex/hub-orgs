use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};

use crate::{
    db::DatabaseClient,
    entities::{invites, sea_orm_active_enums::InviteStatus},
};
#[derive(Default)]
pub struct Query;

#[Object(name = "InvitesQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn invites(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 25)] limit: u64,
        #[graphql(default = 0)] offset: u64,
        status: Option<InviteStatus>,
    ) -> Result<Vec<invites::Model>> {
        let db = &**ctx.data::<DatabaseClient>()?;

        let mut query = invites::Entity::find();

        if let Some(status) = status {
            query = query.filter(invites::Column::Status.eq(status));
        }

        query
            .limit(limit)
            .offset(offset)
            .order_by_desc(invites::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Into::into)
    }
}
