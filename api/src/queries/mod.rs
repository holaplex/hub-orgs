#![allow(clippy::unused_async)] // async-graphql requires the async keyword

pub mod organization;
pub mod project;
pub mod user;
pub mod webhook;

// Add your other ones here to create a unified Query object
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Query(
    organization::Query,
    project::Query,
    user::Query,
    webhook::Query,
);
