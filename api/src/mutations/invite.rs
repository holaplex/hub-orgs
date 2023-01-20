use async_graphql::{self, Context, InputObject, Json, Object, Result};
use sea_orm::{prelude::*, Set};
use uuid::Uuid;

use crate::{
    db::DatabaseClient,
    entities::{invites, sea_orm_active_enums::InviteStatus},
    UserID,
};

#[derive(Default)]

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
        let UserID(id) = ctx.data::<UserID>()?;
        let user_id = id.ok_or_else(|| "no user id")?;
        let db = &**ctx.data::<DatabaseClient>()?;

        let active_model = invites::ActiveModel {
            organization_id: Set(input.organization),
            email: Set(input.email),
            status: Set(InviteStatus::Sent),
            created_by: Set(user_id),
            ..Default::default()
        };

        active_model.insert(db).await.map_err(Into::into)
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
        let db = &**ctx.data::<DatabaseClient>()?;

        let mut active_model: invites::ActiveModel = input.0.into();

        active_model.status = Set(InviteStatus::Accepted);

        active_model.insert(db).await.map_err(Into::into)
    }
}

#[derive(InputObject)]
#[graphql(name = "InviteMemberInput")]
pub struct MemberInput {
    pub organization: Uuid,
    #[graphql(validator(email))]
    pub email: String,
}
