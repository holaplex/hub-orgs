//!

use holaplex_hub_orgs::{
    api::OrgsApi,
    build_schema,
    db::Connection,
    handlers::{graphql_handler, health, playground},
    ory_client::OryClient,
    AppState, Args,
};
use hub_core::anyhow::Context as AnyhowContext;
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};
use poem_openapi::{
    param::Path, payload::Json, ApiResponse, Object, OpenApi, OpenApiService, Tags,
};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-orgs",
    };

    hub_core::run(opts, |common, args| {
        let Args {
            port,
            db,
            ory,
            svix,
        } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();
            let ory_client = OryClient::new(ory);
            let svix_client = svix.build_client();

            let state = AppState::new(schema, connection, ory_client, svix_client);

            let api_service = OpenApiService::new(OrgsApi, "Orgs", "0.1.0")
                .server(format!("http://localhost:{port}/api"));
            let ui = api_service.swagger_ui();
            let spec = api_service.spec_endpoint();

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .nest("/api", api_service.with(AddData::new(state)))
                        .nest("/", ui)
                        .at("/spec", spec)
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
