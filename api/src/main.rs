//!

use holaplex_hub_orgs::{
    build_schema,
    db::Connection,
    handlers::{graphql_handler, health, playground},
    ory_client::OryClient,
    proto, AppState, Args,
};
use hub_core::anyhow::Context as AnyhowContext;
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-orgs",
    };

    hub_core::run(opts, |common, args| {
        let Args { port, db, ory } = args;

        common.rt.block_on(async move {
            let connection = Connection::new(db)
                .await
                .context("failed to get database connection")?;

            let schema = build_schema();
            let ory_client = OryClient::new(ory);

            let producer = common.producer_cfg.build::<proto::Event>().await?;

            let state = AppState::new(schema, connection, ory_client, producer);

            Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
                .run(
                    Route::new()
                        .at("/graphql", post(graphql_handler).with(AddData::new(state)))
                        .at("/playground", get(playground))
                        .at("/health", get(health)),
                )
                .await
                .context("failed to build graphql server")
        })
    });
}
