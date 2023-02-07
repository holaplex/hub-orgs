use hub_core::uuid::Uuid;
use poem::{error::InternalServerError, http::StatusCode, web::Data, Error, Result};
use poem_openapi::{
    param::{Header, Path},
    payload::Json,
    OpenApi,
};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{invites, sea_orm_active_enums::InviteStatus},
    AppState,
};

pub struct Invites;

#[OpenApi]
impl Invites {
    #[oai(path = "/invites", method = "get")]
    async fn get_invites(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
    ) -> Result<Json<Vec<invites::Model>>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let conn = state.connection.get();

        let invites = invites::Entity::find()
            .filter(invites::Column::OrganizationId.eq(organization))
            .all(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(invites))
    }

    #[oai(path = "/invites", method = "post")]
    async fn create_invite(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
        input: Json<invites::Model>,
    ) -> Result<Json<invites::Model>> {
        let Data(state) = state;
        let Header(organization) = organization;
        let Header(user) = user;
        let input = input.0;

        let conn = state.connection.get();

        let active_model = invites::ActiveModel {
            organization_id: Set(organization),
            email: Set(input.email),
            status: Set(InviteStatus::Sent),
            created_by: Set(user),
            ..Default::default()
        };

        let invite = active_model
            .insert(conn)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(invite))
    }

    #[oai(path = "/invites/:invite/accept", method = "put")]
    async fn accept_invite(
        &self,
        state: Data<&AppState>,
        #[oai(name = "X-ORGANIZATION-ID")] organization: Header<Uuid>,
        #[oai(name = "X-USER-ID")] user: Header<Uuid>,
        invite: Path<Uuid>,
    ) -> Result<Json<invites::Model>> {
        let Data(state) = state;
        let Path(id) = invite;
        let Header(_user) = user;
        let Header(organization) = organization;

        let conn = state.connection.get();

        let invite = invites::Entity::find()
            .filter(
                invites::Column::Id
                    .eq(id)
                    .and(invites::Column::OrganizationId.eq(organization)),
            )
            .one(conn)
            .await
            .map_err(InternalServerError)?;

        // TODO: check that the email of the logged in user matches the invite

        let invite = invite.ok_or(Error::from_status(StatusCode::NOT_FOUND))?;
        let mut invite: invites::ActiveModel = invite.into();

        invite.status = Set(InviteStatus::Accepted);

        let invite = invite.insert(conn).await.map_err(InternalServerError)?;

        Ok(Json(invite))
    }
}
