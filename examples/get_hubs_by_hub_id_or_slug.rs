use bump_api::BumpClient;
use bump_api::model::*;
#[tokio::main]
async fn main() {
    let client = BumpClient::from_env();
    let hub_id_or_slug = "your hub id or slug";
    let response = client
        .get_hubs_by_hub_id_or_slug(hub_id_or_slug)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
