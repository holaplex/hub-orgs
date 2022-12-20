pub mod organization;
pub mod project;
pub mod user;

// Add your other ones here to create a unified Query object
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(organization::Query, project::Query, user::Query);
