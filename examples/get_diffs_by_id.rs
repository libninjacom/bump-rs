use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let id = "your id";
    let response = client
        .get_diffs_by_id(id)
        .formats(&["your formats"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
