use bump_api::BumpClient;
use bump_api::model::*;
use bump_api::request::PostValidationsRequired;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let args = PostValidationsRequired {
        documentation: "your documentation",
        hub: "your hub",
        url: "your url",
        auto_create_documentation: true,
        definition: "your definition",
        documentation_name: "your documentation name",
        references: vec![
            Reference { location : Some("your location".to_owned()), content :
            Some("your content".to_owned()) }
        ],
    };
    let response = client.post_validations(args).send().await.unwrap();
    println!("{:#?}", response);
}
