use sea_orm::*;

use super::entities::{organizations, organizations::Entity as Organization};

pub struct Query;

impl Query {
    pub async fn find_organization_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<organizations::Model>, DbErr> {
        Organization::find_by_id(id).one(db).await
    }

    pub async fn get_all_organizations(db: &DbConn) -> Result<Vec<organizations::Model>, DbErr> {
        Organization::find().all(db).await
    }
}
