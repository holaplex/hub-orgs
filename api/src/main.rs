//!

use holaplex_hub_orgs::{
    build_schema,
    db::Connection,
    handlers::{browser_login, browser_organization_select, graphql_handler, health, playground},
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

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at(
                            "/graphql",
                            post(graphql_handler).with(AddData::new(state.clone())),
                        )
                        .at("/playground", get(playground))
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
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
