use hub_core::async_graphql;

pub mod organization;

pub use organization::OrganizationQuery;

// Add your other ones here to create a unified Query object
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(OrganizationQuery);
