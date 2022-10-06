use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostValidationsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostValidationsRequired {
        hub: "your hub",
        definition: "your definition",
        auto_create_documentation: true,
        url: "your url",
        documentation: "your documentation",
        documentation_name: "your documentation name",
        references: vec![
            Reference { location : Some("your location".to_owned()), content :
            Some("your content".to_owned()) }
        ],
    };
    let response = client.post_validations(args).send().await.unwrap();
    println!("{:#?}", response);
}
