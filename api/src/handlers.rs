use hub_core::uuid::Uuid;
use poem::{
    error::InternalServerError,
    handler,
    http::StatusCode,
    web::{
        cookie::{Cookie, CookieJar, SameSite},
        Data, Json, Path,
    },
    Error, FromRequest, Request, RequestBody, Result,
};
use serde::Serialize;

use crate::{
    entities::{members, owners},
    AppState,
};

const HUB_ORG_COOKIE_NAME: &str = "_hub_org";

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

#[derive(Serialize)]
pub struct LoginCompleteResponse {
    pub redirect_path: String,
}

#[handler]
pub async fn browser_login(
    state: Data<&AppState>,
    cookie_jar: &CookieJar,
    user_id: UserId,
) -> Result<Json<LoginCompleteResponse>> {
    let Data(state) = state;
    let conn = state.connection.get();
    let user_id = user_id.0;

    let owners = owners::Entity::find_by_user(user_id)
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let memberships = members::Entity::find_by_user(user_id)
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let organizations: Vec<Uuid> = owners
        .into_iter()
        .map(|o| o.organization_id)
        .chain(memberships.into_iter().map(|m| m.organization_id))
        .collect();

    match organizations.len() {
        0 => Ok(Json(LoginCompleteResponse {
            redirect_path: "/organizations/new".to_string(),
        })),
        1 => {
            let mut cookie =
                Cookie::new_with_str(HUB_ORG_COOKIE_NAME, organizations[0].to_string());
            cookie.set_path("/");
            cookie.set_http_only(Some(true));
            cookie.set_same_site(Some(SameSite::Lax));

            cookie_jar.add(cookie);

            Ok(Json(LoginCompleteResponse {
                redirect_path: "/organization/projects".to_string(),
            }))
        },
        _ => Ok(Json(LoginCompleteResponse {
            redirect_path: "/organizations".to_string(),
        })),
    }
}

#[derive(Serialize)]
pub struct OrganizationSelectResponse {
    pub redirect_path: String,
}

#[handler]
pub async fn browser_organization_select(
    state: Data<&AppState>,
    cookie_jar: &CookieJar,
    user_id: UserId,
    organization: Path<Uuid>,
) -> Result<Json<OrganizationSelectResponse>> {
    let Data(state) = state;
    let Path(organization) = organization;
    let conn = state.connection.get();
    let user_id = user_id.0;

    let owners = owners::Entity::find_by_user(user_id)
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let memberships = members::Entity::find_by_user(user_id)
        .all(conn)
        .await
        .map_err(InternalServerError)?;

    let organizations: Vec<Uuid> = owners
        .into_iter()
        .map(|o| o.organization_id)
        .chain(memberships.into_iter().map(|m| m.organization_id))
        .collect();

    if organizations.contains(&organization) {
        let mut cookie = Cookie::new_with_str(HUB_ORG_COOKIE_NAME, organization.to_string());
        cookie.set_path("/");
        cookie.set_http_only(Some(true));
        cookie.set_same_site(Some(SameSite::Lax));

        cookie_jar.add(cookie);

        Ok(Json(OrganizationSelectResponse {
            redirect_path: "/organization/projects".to_string(),
        }))
    } else {
        Err(Error::from_string(
            "user not affiliated to the organization",
            StatusCode::UNAUTHORIZED,
        ))
    }
}
