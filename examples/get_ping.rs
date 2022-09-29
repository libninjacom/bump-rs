use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let response = client.get_ping().send().await.unwrap();
    println!("{:#?}", response);
}
