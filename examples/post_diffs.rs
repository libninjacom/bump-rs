use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let response = client
        .post_diffs()
        .url("your url")
        .previous_url("your previous url")
        .previous_definition("your previous definition")
        .previous_references(
            vec![
                Reference { location : Some("your location".to_owned()), content :
                Some("your content".to_owned()) }
            ],
        )
        .definition("your definition")
        .references(
            vec![
                Reference { location : Some("your location".to_owned()), content :
                Some("your content".to_owned()) }
            ],
        )
        .expires_at("your expires at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
