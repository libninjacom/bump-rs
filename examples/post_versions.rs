use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostVersionsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostVersionsRequired {
        documentation: "your documentation",
        auto_create_documentation: true,
        unpublished: true,
        branch_name: "your branch name",
        previous_version_id: "your previous version id",
        hub: "your hub",
        documentation_name: "your documentation name",
        definition: "your definition",
        references: vec![
            Reference { location : Some("your location".to_owned()), content :
            Some("your content".to_owned()) }
        ],
    };
    let response = client.post_versions(args).send().await.unwrap();
    println!("{:#?}", response);
}
