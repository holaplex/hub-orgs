use std::str::FromStr;

use poem::{middleware::AddDataEndpoint, test::TestClient};
use sea_orm::{DatabaseBackend, MockDatabase};

use super::*;
use crate::entities::{organizations, projects};

async fn build_client()
-> Result<TestClient<AddDataEndpoint<Route, Schema<Query, Mutation, EmptySubscription>>>> {
    let db = Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection());

    let schema = build_schema(db).await.unwrap();

    let app = Route::new().at("/", graphql_handler).data(schema);
    let cli = poem::test::TestClient::new(app);

    Ok(cli)
}

pub async fn build_schema(db: Arc<DatabaseConnection>) -> Result<AppSchema> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .data(db)
        .finish();

    Ok(schema)
}

#[tokio::test]
pub async fn test_user_id_header() -> Result<()> {
    let cli = build_client().await?;

    let resp = cli
        .post("/")
        .header("X-USER-ID", "258fb83e-91b7-4c3f-9fd9-a131be59c37d")
        .body("{}")
        .send()
        .await;

    resp.assert_status_is_ok();

    Ok(())
}

#[tokio::test]
pub async fn test_organization_query() -> Result<()> {
    let uuid = uuid::Uuid::from_str("b3b2c8db-fcbe-4c8e-bbdf-23644b02f95c")?;
    let time = chrono::NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![organizations::Model {
                id: uuid,
                name: "hello".to_string(),
                created_at: time,
                deactivated_at: None,
            }]])
            .into_connection(),
    );

    let schema = build_schema(db).await?;

    let app = Route::new().at("/", graphql_handler).data(schema);
    let cli = poem::test::TestClient::new(app);

    let resp = cli
        .post("/")
        .header("X-USER-ID", "258fb83e-91b7-4c3f-9fd9-a131be59c37d")
        .header("content-type", "application/json")
        .body(r#"{"query": "query{organizations{id}}"}"#)
        .send()
        .await;

    resp.assert_status_is_ok();
    let json = resp.json().await;
    let json_value = json.value();

    let org = json_value.object().get("data");

    org.assert_not_null();

    let val = org.object().get("organizations");

    val.assert_not_null();

    Ok(())
}

#[tokio::test]
pub async fn test_organization_mutation() -> Result<()> {
    let cli = build_client().await?;

    let mutation = cli
        .post("/")
        .header("X-USER-ID", "258f083e-91b7-4c3f-9fd9-a131be59c37d")
        .header("content-type", "application/json")
        .body(r#"{"query": "mutation{createOrganization(input:{name: \"test\"}){id}}"}"#)
        .send()
        .await;

    mutation.assert_status_is_ok();

    Ok(())
}

#[tokio::test]
pub async fn test_project_query() -> Result<()> {
    let uuid = uuid::Uuid::from_str("b3b2c8db-fcbe-4c8e-bbdf-23644b02f95c")?;
    let time = chrono::NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;

    let db = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![projects::Model {
                id: uuid,
                name: "hello".to_string(),
                organization_id: uuid,
                created_at: time,
                deactivated_at: None,
            }]])
            .into_connection(),
    );

    let schema = build_schema(db).await?;

    let app = Route::new().at("/", graphql_handler).data(schema);
    let cli = poem::test::TestClient::new(app);

    let resp = cli
        .post("/")
        .header("X-USER-ID", "258fb83e-91b7-4c3f-9fd9-a131be59c37d")
        .header("content-type", "application/json")
        .body(r#"{"query": "query{projects(limit:10, offset:0){id}}"}"#)
        .send()
        .await;

    resp.assert_status_is_ok();
    let json = resp.json().await;
    let json_value = json.value();

    let org = json_value.object().get("data");

    org.assert_not_null();

    let val = org.object().get("projects");

    val.assert_not_null();

    Ok(())
}
