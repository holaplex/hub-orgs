#![allow(clippy::unused_async)] // async-graphql requires the async keyword

mod invite;
mod organization;
mod project;
mod user;
mod webhook;

// Add your other ones here to create a unified Query object
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Query(
    organization::Query,
    project::Query,
    user::Query,
    invite::Query,
    webhook::Query,
);
