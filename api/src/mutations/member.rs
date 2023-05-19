// Import necessary dependencies
use async_graphql::{Context, Error, InputObject, Object, Result};
use hub_core::{chrono::Utc, producer::Producer};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::members::{self, Member},
    proto::{self, organization_events::Event, OrganizationEventKey, OrganizationEvents},
    AppContext,
};

// Define a struct for GraphQL mutation
#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

// Implement the GraphQL mutation
#[Object(name = "MemberMutation")]
impl Mutation {
    // Define a GraphQL mutation to deactivate a member
    /// Returns member object on success
    ///
    /// # Errors
    /// This code may result in an error if the update to the database fails or if it fails to produce an event.
    pub async fn deactivate_member(
        &self,
        ctx: &Context<'_>,
        input: DeactivateMemberInput,
    ) -> Result<Member> {
        // Get AppContext and Producer instances from the Context object
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        // Find a member by ID
        let member = members::Entity::find_by_id(input.id)
            .one(db.get())
            .await?
            .ok_or_else(|| Error::new("member not found"))?;

        // Convert the member to ActiveModel and update the deactivated_at field
        let mut member_am: members::ActiveModel = member.into();
        member_am.deactivated_at = Set(Some(Utc::now().into()));

        // Update the member and return it
        let member = member_am.update(db.get()).await?;

        // Send an event to a message queue
        let event = OrganizationEvents {
            event: Some(Event::MemberDeactivated(proto::Member {
                organization_id: member.organization_id.to_string(),
            })),
        };
        let key = OrganizationEventKey {
            id: member.id.to_string(),
            user_id: member.user_id.to_string(),
        };
        producer.send(Some(&event), Some(&key)).await?;

        Ok(member.into())
    }

    // Define a GraphQL mutation to reactivate a member
    /// Returns member object on success
    ///
    /// # Errors
    /// This code may result in an error if the update to the database fails or if it fails to produce an event.
    pub async fn reactivate_member(
        &self,
        ctx: &Context<'_>,
        input: ReactivateMemberInput,
    ) -> Result<Member> {
        // Get AppContext and Producer instances from the Context object
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        // Find a member by ID
        let member = members::Entity::find_by_id(input.id)
            .one(db.get())
            .await?
            .ok_or_else(|| Error::new("member not found"))?;

        // Convert the member to ActiveModel and clear the deactivated_at field
        let mut member_am: members::ActiveModel = member.into();
        member_am.deactivated_at = Set(None);

        // Update the member and return it
        let member = member_am.update(db.get()).await?;

        // Send an event to a message queue
        let event = OrganizationEvents {
            event: Some(Event::MemberReactivated(proto::Member {
                organization_id: member.organization_id.to_string(),
            })),
        };
        let key = OrganizationEventKey {
            id: member.id.to_string(),
            user_id: member.user_id.to_string(),
        };
        producer.send(Some(&event), Some(&key)).await?;

        Ok(member.into())
    }
}

// Define the input object for the "deactivate_member" mutation
#[derive(InputObject, Debug)]
pub struct DeactivateMemberInput {
    pub id: Uuid,
}

// Define the input object for the "reactivate_member" mutation
#[derive(InputObject, Debug)]
pub struct ReactivateMemberInput {
    pub id: Uuid,
}
