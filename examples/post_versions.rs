use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostVersionsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostVersionsRequired {
        documentation_name: "your documentation name",
        hub: "your hub",
        references: vec![
            Reference { content : Some("your content".to_owned()), location :
            Some("your location".to_owned()) }
        ],
        auto_create_documentation: true,
        documentation: "your documentation",
        definition: "your definition",
        branch_name: "your branch name",
        previous_version_id: "your previous version id",
        unpublished: true,
    };
    let response = client.post_versions(args).send().await.unwrap();
    println!("{:#?}", response);
}
