pub mod credential;
pub mod invite;
pub mod organization;
pub mod project;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(OrganizationMutation, OtherMutation, OtherOtherMutation)
#[derive(Debug, async_graphql::MergedObject, Default)]
pub struct Mutation(
    organization::Mutation,
    project::Mutation,
    invite::Mutation,
    credential::Mutation,
);
