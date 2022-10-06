use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let definition = "your definition";
    let response = client
        .post_previews(definition)
        .references(
            vec![
                Reference { location : Some("your location".to_owned()), content :
                Some("your content".to_owned()) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
