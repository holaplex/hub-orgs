use std::str::FromStr;

use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use hub_core::uuid::Uuid;
use regex::Regex;

use crate::{entities::projects::Model, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "CredentialQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_credential_by_client_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] client_id: String,
        audiences: Option<Vec<String>>,
    ) -> Result<Credential> {
        Ok(Credential {
            client_id,
            audiences: audiences.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, SimpleObject, Default)]
#[graphql(complex)]
pub struct Credential {
    pub client_id: String,
    #[graphql(external)]
    pub audiences: Vec<String>,
}

#[ComplexObject]
impl Credential {
    #[graphql(requires = "audiences")]
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Model>> {
        let AppContext { project_loader, .. } = ctx.data::<AppContext>()?;

        let project_ids = self
            .audiences
            .clone()
            .into_iter()
            .map(|audience| {
                let regex = Regex::new(
                    r#"https:\\/\\/holaplex\\.com\\/projects\\/(\w{8}-\w{4}-\w{4}-\w{4}-\w{12})"#,
                )?;
                let uuid = regex.replace(&audience, "").into_owned();

                Ok(Uuid::from_str(&uuid)?)
            })
            .collect::<Result<Vec<Uuid>>>();

        let projects = project_loader
            .load_many(project_ids?)
            .await?
            .into_values()
            .collect();

        Ok(projects)
    }
}