use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject};
use hub_core::{chrono::Utc, producer::Producer};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{invites, members, sea_orm_active_enums::InviteStatus},
    proto::{organization_events::Event, Member, OrganizationEventKey, OrganizationEvents},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "InviteMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn invite_member(
        &self,
        ctx: &Context<'_>,
        input: MemberInput,
    ) -> Result<invites::Model> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let active_model = invites::ActiveModel {
            organization_id: Set(input.organization),
            email: Set(input.email),
            status: Set(InviteStatus::Sent),
            created_by: Set(user_id),
            ..Default::default()
        };

        active_model.insert(db.get()).await.map_err(Into::into)
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
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

        validate_email_match(&(invite.email.clone(), user_email))?;

        let mut active_model: invites::ActiveModel = invite.into();

        active_model.status = Set(InviteStatus::Accepted);
        active_model.updated_at = Set(Some(Utc::now().naive_utc()));

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

#[derive(InputObject, Debug)]
#[graphql(name = "InviteMemberInput")]
pub struct MemberInput {
    pub organization: Uuid,
    #[graphql(validator(email))]
    pub email: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct AcceptInviteInput {
    pub invite: Uuid,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct AcceptInvitePayload {
    pub invite: invites::Model,
}

fn validate_email_match(emails: &(String, String)) -> Result<()> {
    if emails.0 == emails.1 {
        return Ok(());
    }
    Err(Error::new("user email does not match the invite"))
}
