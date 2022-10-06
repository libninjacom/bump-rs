use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostVersionsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostVersionsRequired {
        definition: "your definition",
        documentation_name: "your documentation name",
        documentation: "your documentation",
        previous_version_id: "your previous version id",
        branch_name: "your branch name",
        unpublished: true,
        hub: "your hub",
        auto_create_documentation: true,
        references: vec![
            Reference { location : Some("your location".to_owned()), content :
            Some("your content".to_owned()) }
        ],
    };
    let response = client.post_versions(args).send().await.unwrap();
    println!("{:#?}", response);
}
