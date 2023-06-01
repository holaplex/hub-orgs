use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::{chrono::Utc, producer::Producer};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{invites, members, organizations, sea_orm_active_enums::InviteStatus},
    proto::{organization_events::Event, Invite, Member, OrganizationEventKey, OrganizationEvents},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "InviteMutation")]
impl Mutation {
    /// To invite a person to the organization, provide their email address.
    /// # Error
    /// This mutation will produce an error if it is unable to connect to the database or if there is no associated user set in the X-USER-ID header.
    pub async fn invite_member(
        &self,
        ctx: &Context<'_>,
        input: MemberInput,
    ) -> Result<invites::Model> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let invite = invites::Entity::find()
            .filter(invites::Column::Email.eq(input.email.clone()))
            .filter(invites::Column::OrganizationId.eq(input.organization))
            .one(db.get())
            .await?;

        if invite.is_some() {
            return Err(Error::new("Invite already exists"));
        }

        let organization = organizations::Entity::find_by_id(input.organization)
            .one(db.get())
            .await?
            .ok_or_else(|| Error::new("organization not found"))?;

        let active_model = invites::ActiveModel {
            organization_id: Set(input.organization),
            email: Set(input.email.to_lowercase()),
            status: Set(InviteStatus::Sent),
            created_by: Set(user_id),
            ..Default::default()
        };

        let event = OrganizationEvents {
            event: Some(Event::InviteCreated(Invite {
                organization: organization.name,
                email: input.email.to_lowercase(),
            })),
        };

        let key = OrganizationEventKey {
            id: input.organization.to_string(),
            user_id: user_id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        active_model.insert(db.get()).await.map_err(Into::into)
    }

    /// Accept an invite to the organization.
    /// # Error
    /// This mutation will produce an error if it is unable to connect to the database or if the user's email does not match the invitation.
    pub async fn accept_invite(
        &self,
        ctx: &Context<'_>,
        input: AcceptInviteInput,
    ) -> Result<AcceptInvitePayload> {
        let AppContext {
            db,
            user_id,
            user_email,
            ..
        } = ctx.data::<AppContext>()?;
        let conn = db.get();
        let producer = ctx.data::<Producer<OrganizationEvents>>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;
        let user_email = user_email
            .clone()
            .ok_or_else(|| Error::new("X-EMAIL-ID header not found"))?;

        let invite = invites::Entity::find()
            .filter(invites::Column::Id.eq(input.invite))
            .one(conn)
            .await?
            .ok_or_else(|| Error::new("invite not found"))?;

        validate_email_match(&(invite.email.to_lowercase(), user_email))?;

        let mut active_model: invites::ActiveModel = invite.into();

        active_model.status = Set(InviteStatus::Accepted);
        active_model.updated_at = Set(Some(Utc::now().into()));

        let invite = active_model.update(conn).await?;

        let member = members::ActiveModel {
            user_id: Set(user_id),
            organization_id: Set(invite.organization_id),
            invite_id: Set(invite.id),
            ..Default::default()
        };

        let member_model = member.insert(conn).await?;

        let event = OrganizationEvents {
            event: Some(Event::MemberAdded(Member {
                organization_id: invite.organization_id.to_string(),
            })),
        };

        let key = OrganizationEventKey {
            id: member_model.id.to_string(),
            user_id: user_id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(AcceptInvitePayload { invite })
    }
}

/// Input required for inviting a member to the organization.
#[derive(InputObject, Debug)]
#[graphql(name = "InviteMemberInput")]
pub struct MemberInput {
    /// The ID of the organization.
    pub organization: Uuid,
    /// The email address of the invited user.
    #[graphql(validator(email))]
    pub email: String,
}

/// Input required for accepting an invitation to the organization.
#[derive(Debug, Clone, InputObject)]
pub struct AcceptInviteInput {
    /// The ID of the invitation.
    pub invite: Uuid,
}

/// The response returned after accepting an invitation to the organization.
#[derive(Debug, Clone, SimpleObject)]
pub struct AcceptInvitePayload {
    /// The invitation to the organization that has been accepted.
    pub invite: invites::Model,
}

fn validate_email_match(emails: &(String, String)) -> Result<()> {
    if emails.0 == emails.1 {
        return Ok(());
    }
    Err(Error::new("user email does not match the invite"))
}
