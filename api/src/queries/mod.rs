#![allow(clippy::unused_async)] // async-graphql requires the async keyword

pub mod invites;
pub mod organization;
pub mod project;
pub mod user;

// Add your other ones here to create a unified Query object
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Query(
    organization::Query,
    project::Query,
    user::Query,
    invites::Query,
);
