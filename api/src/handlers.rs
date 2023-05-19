use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use hub_core::uuid::Uuid;
use poem::{
    error::InternalServerError,
    handler,
    http::StatusCode,
    web::{
        cookie::{Cookie, CookieJar, SameSite},
        Data, Html, Json, Path,
    },
    Error, IntoResponse, Result,
};
use serde::Serialize;

use crate::{
    entities::{members, owners},
    AppContext, AppState, UserEmail, UserID,
};

const HUB_ORG_COOKIE_NAME: &str = "_hub_org";

#[handler]
pub fn health() {}

#[handler]
pub fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[handler]
pub async fn graphql_handler(
    Data(state): Data<&AppState>,
    user_id: UserID,
    user_email: UserEmail,
    req: GraphQLRequest,
) -> Result<GraphQLResponse> {
    let UserID(user_id) = user_id;
    let UserEmail(user_email) = user_email;

    let context = AppContext::new(
        state.connection.clone(),
        user_id,
        user_email.map(|e| e.to_lowercase()),
    );

    Ok(state
        .schema
        .execute(
            req.0
                .data(context)
                .data(state.producer.clone())
                .data(state.asset_proxy.clone()),
        )
        .await
        .into())
}

#[derive(Serialize)]
pub struct LoginCompleteResponse {
    pub redirect_path: String,
}

#[handler]
pub async fn browser_login(
    state: Data<&AppState>,
    cookie_jar: &CookieJar,
    user_id: UserID,
) -> Result<Json<LoginCompleteResponse>> {
    let Data(state) = state;
    let conn = state.connection.get();
    let UserID(user_id) = user_id;
    let user_id = user_id
        .ok_or_else(|| Error::from_string("X-USER-ID not found", StatusCode::BAD_REQUEST))?;

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
                redirect_path: "/projects".to_string(),
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
    user_id: UserID,
    organization: Path<Uuid>,
) -> Result<Json<OrganizationSelectResponse>> {
    let Data(state) = state;
    let Path(organization) = organization;
    let UserID(user_id) = user_id;
    let conn = state.connection.get();
    let user_id = user_id
        .ok_or_else(|| Error::from_string("X-USER-ID not found", StatusCode::BAD_REQUEST))?;

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
            redirect_path: "/projects".to_string(),
        }))
    } else {
        Err(Error::from_string(
            "user not affiliated to the organization",
            StatusCode::UNAUTHORIZED,
        ))
    }
}
