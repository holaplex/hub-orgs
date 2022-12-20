use async_graphql::{self, Context, Object, Result};
use sea_orm::{prelude::*, QueryOrder};

use crate::entities::projects;

#[derive(Default)]
pub struct Query;

#[Object(name = "ProjectQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn projects(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] page_number: u64,
        #[graphql(default = 25)] page_size: u64,
    ) -> Result<Vec<projects::Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let pages = projects::Entity::find()
            .order_by_asc(projects::Column::CreatedAt)
            .paginate(db, page_size);

        pages.fetch_page(page_number).await.map_err(Into::into)
    }
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    async fn project(&self, ctx: &Context<'_>, id: uuid::Uuid) -> Result<Option<projects::Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();

        projects::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(Into::into)
    }
}
