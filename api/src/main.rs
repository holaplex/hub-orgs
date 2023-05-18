//!

use holaplex_hub_orgs::{
    build_schema,
    db::Connection,
    handlers::{browser_login, browser_organization_select, graphql_handler, health, playground},
    proto, AppState, Args,
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
        let Args { port, db } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();
            let producer = common
                .producer_cfg
                .build::<proto::OrganizationEvents>()
                .await?;

            let state = AppState::new(schema, connection, producer, common.asset_proxy);

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
