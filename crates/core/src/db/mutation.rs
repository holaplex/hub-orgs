use sea_orm::*;

use super::entities::{organizations, organizations::Entity as Organization};

pub struct Mutation;

impl Mutation {
    pub async fn create_organization(
        db: &DbConn,
        form_data: organizations::Model,
    ) -> Result<organizations::Model, DbErr> {
        let active_model = organizations::ActiveModel {
            name: Set(form_data.name.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            deactivated_at: Set(form_data.deactivated_at.to_owned()),
            ..Default::default()
        };
        let res = Organization::insert(active_model).exec(db).await?;

        Ok(organizations::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn update_organization(
        db: &DbConn,
        id: i32,
        form_data: organizations::Model,
    ) -> Result<organizations::Model, DbErr> {
        let org: organizations::ActiveModel = Organization::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("Cannot find organization.".to_owned()))
            .map(Into::into)?;

        organizations::ActiveModel {
            id: org.id,
            name: Set(form_data.name.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            deactivated_at: Set(form_data.deactivated_at.to_owned()),
        }
        .update(db)
        .await
    }
}
