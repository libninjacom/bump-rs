use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Hub {
    ///The name of the Hub
    pub name: Option<String>,
    ///The description of the Hub
    pub description: Option<String>,
    ///The public URL of the hub
    pub url: Option<String>,
    ///Creation date of this Hub
    pub created: Option<serde_json::Value>,
    ///Last udpate date of this Hub
    pub modified: Option<serde_json::Value>,
    ///The list of APIs belonging to this Hub
    pub apis: Option<Vec<Api>>,
}
impl std::fmt::Display for Hub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Api {
    ///UUID of this API
    pub id: Option<String>,
    ///Name of this API
    pub name: Option<String>,
    ///Description of this API from the latest definition
    pub description: Option<String>,
    ///Slug of this API
    pub slug: Option<String>,
    ///public documentation URL
    pub url: Option<String>,
    ///Version of this API taken from the latest definition
    pub version: Option<String>,
    ///Extra properties attached to this API
    pub properties: Option<Vec<serde_json::Value>>,
    ///Creation date of this API
    pub created: Option<serde_json::Value>,
    ///Last udpate date of this API
    pub modified: Option<serde_json::Value>,
}
impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Reference {
    ///Location of the external reference as it's called from `definition`, without the relative path (the part after `#/`). Can be an URL of a file system path.
    pub location: Option<String>,
    ///Content of the external reference, as a string.
    pub content: Option<String>,
}
impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    ///Human readable error message.
    pub message: Option<String>,
    ///Hash of invalid attributes with their error messages.
    pub errors: Option<serde_json::Value>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pong {
    ///Sentence about ping and pong
    pub pong: Option<String>,
}
impl std::fmt::Display for Pong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Preview {
    ///Unique id for the preview URL: `https://bump.sh/preview/:id`.
    pub id: Option<String>,
    ///Preview expiration date and time.
    pub expires_at: Option<String>,
    ///The public URL where the preview will be visible.
    pub public_url: Option<String>,
}
impl std::fmt::Display for Preview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Version {
    ///Unique id of your version.
    pub id: Option<String>,
    ///Unique id of your documentation.
    pub doc_id: Option<String>,
    ///The public URL of your documentation.
    pub doc_public_url: Option<String>,
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WithDiff {
    ///Details of each change as a list of diff items
    pub diff_details: Option<Vec<DiffItem>>,
    ///The comparaison diff summary in markdown format
    pub diff_markdown: Option<String>,
    ///The comparaison diff summary
    pub diff_summary: Option<String>,
    ///The public URL of your diff
    pub diff_public_url: Option<String>,
    ///Identifies if the diff includes breaking changes
    pub diff_breaking: Option<bool>,
}
impl std::fmt::Display for WithDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Diff {
    ///Unique id of your diff
    pub id: Option<String>,
    ///The title of the last parsed definition
    pub title: Option<String>,
    ///The public URL of your diff
    pub public_url: Option<String>,
    ///Identifies if the diff includes breaking changes
    pub breaking: Option<bool>,
    ///Details of each change as a list of diff items
    pub details: Option<Vec<DiffItem>>,
    ///URL of previous version specification, in JSON format
    pub previous_version_url: Option<String>,
    ///URL of current version specification, in JSON format
    pub current_version_url: Option<String>,
}
impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffItem {
    ///The identifier of the diff change
    pub id: Option<String>,
    ///The human name of diff change
    pub name: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "type")]
    ///The object type of the diff change
    pub type_: Option<String>,
    ///Identifies if the item is a breaking change
    pub breaking: Option<bool>,
    ///A list of children item changes
    pub children: Option<Vec<DiffItem>>,
}
impl std::fmt::Display for DiffItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffForApi(pub Diff, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocumentationRequest {
    ///UUID or slug of the documentation.
    pub documentation: String,
    ///UUID or slug of the hub if the documentation is part of a hub.
    pub hub: Option<String>,
    ///Name of the documentation to create. Used only if `auto_create_documentation` is set.
    pub documentation_name: Option<String>,
    ///Create the documentation if it does not exist yet. Must be used with a `hub` and a `documentation`.
    pub auto_create_documentation: Option<bool>,
}
impl std::fmt::Display for DocumentationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Validation {
    ///Specification of the given definition as a path: `speficiation_name/specification_version/format`.
    pub specification: Option<String>,
}
impl std::fmt::Display for Validation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionRequest {
    /**Serialized definition of the version. This should be an OpenAPI 2.x, 3.x or AsyncAPI 2.x file serialized as a string, in YAML or JSON.
*/
    pub definition: String,
    ///Import external references used by `definition`. It's usually resources not accessible by Bump servers, like local files or internal URLs.
    pub references: Option<Vec<Reference>>,
    /**Select a branch for this new version (branch will be created if it doesn't exist).

*Defaults to the main branch*.
*/
    pub branch_name: Option<String>,
}
impl std::fmt::Display for VersionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PreviewRequest {
    /****Required** if [`definition`](#post-validations-definition) is not present.
Target definition URL. It should be accessible through HTTP by Bump servers.
*/
    pub url: Option<String>,
    /****Required** if [`url`](#post-validations-url) is not present.
Serialized definition. This should be an OpenAPI 2.x, 3.x or AsyncAPI 2.x file
serialized as a string, in YAML or JSON.
*/
    pub definition: Option<String>,
    ///Import external references used by `definition`. It's usually resources not accessible by Bump servers, like local files or internal URLs.
    pub references: Option<Vec<Reference>>,
}
impl std::fmt::Display for PreviewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffRequest {
    /****Required** if `definition` is not present.
Current definition URL. It should be accessible through HTTP by Bump servers.
*/
    pub url: Option<String>,
    /****Required** if `definition` is not present.
Previous definition URL. It should be accessible through HTTP by Bump servers.
*/
    pub previous_url: Option<String>,
    /****Required** if `url` is not present.
Serialized definition of the previous version. This should be an OpenAPI 2.x, 3.x or AsyncAPI 2.x file
serialized as a string, in YAML or JSON.
*/
    pub previous_definition: Option<String>,
    ///Import external references used by `previous_definition`. It's usually resources not accessible by Bump servers, like local files or internal URLs.
    pub previous_references: Option<Vec<Reference>>,
    /****Required** if `url` is not present.
Serialized definition of the current version. This should be an OpenAPI 2.x, 3.x or AsyncAPI 2.x file
serialized as a string, in YAML or JSON.
*/
    pub definition: Option<String>,
    ///Import external references used by `definition`. It's usually resources not accessible by Bump servers, like local files or internal URLs.
    pub references: Option<Vec<Reference>>,
    ///Public change expiration date and time. After this date, this documentation change will be destroyed.
    pub expires_at: Option<String>,
}
impl std::fmt::Display for DiffRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
