use async_graphql::{self, Context, Error, InputObject, Json, Object, Result};
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{invites, sea_orm_active_enums::InviteStatus},
    AppContext, UserID,
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
        let UserID(id) = user_id;
        let user_id = id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

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
        input: Json<invites::Model>,
    ) -> Result<invites::Model> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        let mut active_model: invites::ActiveModel = input.0.into();

        active_model.status = Set(InviteStatus::Accepted);

        active_model.insert(db.get()).await.map_err(Into::into)
    }
}

#[derive(InputObject, Debug)]
#[graphql(name = "InviteMemberInput")]
pub struct MemberInput {
    pub organization: Uuid,
    #[graphql(validator(email))]
    pub email: String,
}
