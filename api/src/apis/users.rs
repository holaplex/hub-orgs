use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{param::Header, payload::Json, OpenApi};
use sea_orm::{prelude::*, QueryOrder};

use crate::{
    entities::{members, owners},
    AppState,
};

#[derive(Debug, PartialEq, poem_openapi::Union)]
#[oai(discriminator_name = "type")]
enum Affiliation {
    Owner(owners::Model),
    Member(members::Model),
}

pub struct Users;

#[OpenApi]
impl Users {
    #[oai(path = "/affiliations", method = "get")]
    async fn list_user_affiliations(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
    ) -> Result<Json<Vec<Affiliation>>> {
        let Header(user) = user;
        let Data(state) = state;
        let conn = state.connection.get();

        let org_owners = owners::Entity::find()
            .filter(owners::Column::UserId.eq(user))
            .order_by_desc(owners::Column::CreatedAt)
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        let org_members = members::Entity::find()
            .filter(
                members::Column::UserId
                    .eq(user)
                    .and(members::Column::RevokedAt.is_null()),
            )
            .order_by_desc(owners::Column::CreatedAt)
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(
            org_owners
                .into_iter()
                .map(Affiliation::Owner)
                .chain(org_members.into_iter().map(Affiliation::Member))
                .collect(),
        ))
    }
}
