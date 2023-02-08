//!

use holaplex_hub_orgs::{
    apis,
    db::Connection,
    handlers::{browser_login, browser_organization_select, health},
    ory_client::OryClient,
    AppState, Args,
};
use hub_core::anyhow::Context as AnyhowContext;
use poem::{
    get,
    listener::TcpListener,
    middleware::{AddData, CookieJarManager, Cors},
    post, EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;

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

            let ory_client = OryClient::new(ory);
            let svix_client = svix.build_client();

            let state = AppState::new(connection, ory_client, svix_client);

            let api_service = OpenApiService::new(
                (
                    apis::Organizations,
                    apis::Projects,
                    apis::Users,
                    apis::Members,
                    apis::Credentials,
                    apis::Invites,
                    apis::Webhooks,
                ),
                "Orgs",
                "0.1.0",
            )
            .server(format!("http://localhost:{port}/v1"));
            let ui = api_service.swagger_ui();
            let spec = api_service.spec_endpoint();

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .nest("/v1", api_service.with(AddData::new(state.clone())))
                        .nest("/", ui)
                        .nest(
                            "/browser",
                            Route::new()
                                .at("/login", post(browser_login))
                                .at(
                                    "/organizations/:organization/select",
                                    post(browser_organization_select),
                                )
                                .with(AddData::new(state))
                                .with(Cors::new().allow_credentials(true))
                                .with(CookieJarManager::new()),
                        )
                        .at("/spec", spec)
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
