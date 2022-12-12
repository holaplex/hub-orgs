use hub_core::async_graphql;
pub mod objects;
pub mod organization;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(OrganizationMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(organization::OrganizationMutation);
