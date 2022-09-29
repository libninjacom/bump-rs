use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let version_id = "your version id";
    let response = client.get_versions_by_version_id(version_id).send().await.unwrap();
    println!("{:#?}", response);
}
