use hub_core::uuid::Uuid;
use poem::{
    error::InternalServerError,
    handler,
    http::StatusCode,
    web::{
        cookie::{Cookie, CookieJar},
        Data, Redirect,
    },
    Error, FromRequest, Request, RequestBody, Result,
};
use sea_orm::prelude::*;

use crate::{
    entities::{members, owners},
    AppState, Fqdn,
};

#[handler]
pub fn health() {}

pub struct UserId(Uuid);

#[poem::async_trait]
impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let user_id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("missing user id", StatusCode::BAD_REQUEST))?;

        let user_id = Uuid::parse_str(user_id)
            .map_err(|_| Error::from_string("invalid uuid", StatusCode::BAD_REQUEST))?;

        Ok(UserId(user_id))
    }
}

#[handler]
pub async fn login_callback(
    state: Data<&AppState>,
    fqdn: Data<&Fqdn>,
    cookie_jar: &CookieJar,
    user_id: UserId,
) -> Result<Redirect> {
    let Data(state) = state;
    let conn = state.connection.get();
    let Data(fqdn) = fqdn;
    let user_id = user_id.0;

    let owners = owners::Entity::find()
        .filter(owners::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let memberships = members::Entity::find()
        .filter(members::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let organizations: Vec<Uuid> = owners
        .into_iter()
        .map(|o| o.organization_id)
        .chain(memberships.into_iter().map(|m| m.organization_id))
        .collect();

    if organizations.len() == 1 {
        cookie_jar.add(Cookie::new("_hub_org", organizations[0]));

        Ok(Redirect::see_other(format!("{fqdn}/projects")))
    } else {
        Ok(Redirect::see_other(format!("{fqdn}/organizations")))
    }
}
