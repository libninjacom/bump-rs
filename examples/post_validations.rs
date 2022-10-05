use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostValidationsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostValidationsRequired {
        documentation: "your documentation",
        references: vec![
            Reference { content : Some("your content".to_owned()), location :
            Some("your location".to_owned()) }
        ],
        hub: "your hub",
        documentation_name: "your documentation name",
        auto_create_documentation: true,
        url: "your url",
        definition: "your definition",
    };
    let response = client.post_validations(args).send().await.unwrap();
    println!("{:#?}", response);
}
