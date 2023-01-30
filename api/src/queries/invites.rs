use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder, QuerySelect};

use crate::{
    entities::{invites, sea_orm_active_enums::InviteStatus},
    AppContext,
};
#[derive(Debug, Clone, Copy, Default)]
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
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        let mut query = invites::Entity::find();

        if let Some(status) = status {
            query = query.filter(invites::Column::Status.eq(status));
        }

        query
            .limit(limit)
            .offset(offset)
            .order_by_desc(invites::Column::CreatedAt)
            .all(db.get())
            .await
            .map_err(Into::into)
    }
}
