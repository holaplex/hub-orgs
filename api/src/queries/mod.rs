pub mod invites;
pub mod organization;
pub mod project;
pub mod user;
pub mod webhook;

// Add your other ones here to create a unified Query object
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(
    organization::Query,
    project::Query,
    user::Query,
    invites::Query,
    webhook::Query,
);
