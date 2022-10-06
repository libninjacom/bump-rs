//! [`BumpClient`](struct.BumpClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct BumpClient {
    pub(crate) client: httpclient::Client,
    authentication: BumpAuthentication,
}
impl BumpClient {
    pub fn from_env() -> Self {
        let url = "https://bump.sh/api/v1".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: BumpAuthentication::from_env(),
        }
    }
}
impl BumpClient {
    pub fn new(url: &str, authentication: BumpAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: BumpAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            BumpAuthentication::AuthorizationToken { authorization_token } => {
                r = r.token_auth(authorization_token);
            }
            BumpAuthentication::BasicToken { basic_token } => {
                r = r.basic_auth(basic_token);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Create a diff

Create a diff between any two given API definitions.
The diff result will be available asynchronously and needs to be retrieved with the [`GET /diffs/:id` API endpoint](#operation-get-diffs-parameter).
*/
    pub fn post_diffs(&self) -> request::PostDiffsRequest {
        request::PostDiffsRequest {
            client: &self,
            url: None,
            previous_url: None,
            previous_definition: None,
            previous_references: None,
            definition: None,
            references: None,
            expires_at: None,
        }
    }
    /**Fetch detailed information from an existing diff

Fetch the result of a previously created diff with the [`POST /diffs` API endpoint](#operation-post-diffs).
*/
    pub fn get_diffs_by_id(&self, id: &str) -> request::GetDiffsByIdRequest {
        request::GetDiffsByIdRequest {
            client: &self,
            id: id.to_owned(),
            formats: None,
        }
    }
    /**Fetch information of an existing Hub

Fetch information of an existing Hub including the list of APIs it contains. The response follows the [APIs.json specification](http://apisjson.org/)
*/
    pub fn get_hubs_by_hub_id_or_slug(
        &self,
        hub_id_or_slug: &str,
    ) -> request::GetHubsByHubIdOrSlugRequest {
        request::GetHubsByHubIdOrSlugRequest {
            client: &self,
            hub_id_or_slug: hub_id_or_slug.to_owned(),
        }
    }
    /**Create a new version

Deploy a new version for a given documentation, which will become the current version.
*/
    pub fn post_versions(
        &self,
        args: request::PostVersionsRequired,
    ) -> request::PostVersionsRequest {
        request::PostVersionsRequest {
            client: &self,
            documentation: args.documentation.to_owned(),
            hub: args.hub.to_owned(),
            documentation_name: args.documentation_name.to_owned(),
            auto_create_documentation: args.auto_create_documentation,
            definition: args.definition.to_owned(),
            references: args.references,
            branch_name: args.branch_name.to_owned(),
            previous_version_id: args.previous_version_id.to_owned(),
            unpublished: args.unpublished,
        }
    }
    /**Validate a documentation definition

Validate a definition against its schema (OpenAPI or AsyncAPI) and return errors without creating a new version. This is useful in a CI process, to validate that a changed definition file is valid and won't fail when being deployed on Bump.
*/
    pub fn post_validations(
        &self,
        args: request::PostValidationsRequired,
    ) -> request::PostValidationsRequest {
        request::PostValidationsRequest {
            client: &self,
            documentation: args.documentation.to_owned(),
            hub: args.hub.to_owned(),
            documentation_name: args.documentation_name.to_owned(),
            auto_create_documentation: args.auto_create_documentation,
            url: args.url.to_owned(),
            definition: args.definition.to_owned(),
            references: args.references,
        }
    }
    /**Create a preview

Create a preview for a given documentation file. The preview will have a unique temporary URL, and will be active for 30 minutes.
*/
    pub fn post_previews(&self, definition: &str) -> request::PostPreviewsRequest {
        request::PostPreviewsRequest {
            client: &self,
            definition: definition.to_owned(),
            references: None,
        }
    }
    /**Update an existing preview

Update a preview with the given documentation file. The preview will stay active for 30 minutes after the last update.
*/
    pub fn put_previews_by_preview_id(
        &self,
        preview_id: &str,
        definition: &str,
    ) -> request::PutPreviewsByPreviewIdRequest {
        request::PutPreviewsByPreviewIdRequest {
            client: &self,
            preview_id: preview_id.to_owned(),
            definition: definition.to_owned(),
            references: None,
        }
    }
    /**Fetch a full documentation version including diff summary

Fetch a full documentation version including diff summary.
*/
    pub fn get_versions_by_version_id(
        &self,
        version_id: &str,
    ) -> request::GetVersionsByVersionIdRequest {
        request::GetVersionsByVersionIdRequest {
            client: &self,
            version_id: version_id.to_owned(),
        }
    }
    /**Check the API status

Responds a pong if the API is up and running.*/
    pub fn get_ping(&self) -> request::GetPingRequest {
        request::GetPingRequest {
            client: &self,
        }
    }
}
pub enum BumpAuthentication {
    AuthorizationToken { authorization_token: String },
    BasicToken { basic_token: String },
}
impl BumpAuthentication {
    pub fn from_env() -> Self {
        Self::AuthorizationToken {
            authorization_token: std::env::var("BUMP_AUTHORIZATION_TOKEN")
                .expect("Environment variable BUMP_AUTHORIZATION_TOKEN is not set."),
        }
    }
}
