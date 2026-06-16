use dotenvy::dotenv;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Listing {
    id: String,
    price: u64,
    item: Item,
}
#[derive(Deserialize)]
struct Item {
    float_value: f64,
    is_stattrak: bool,
}

async fn get_bulk_price(
    market_hash_name: String,
    count: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    // Takes hash name of a target item and count, finds X (count) cheapest of specified item
    // and calculates the average price (generic search)
    let base_url = "https://csfloat.com/api/v1/listings";
    let mut headers = reqwest::header::HeaderMap::new();
    dotenv().ok();
    let api_key = std::env::var("CSFLOAT_API_KEY").expect("CSFLOAT_API_KEY not set");
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&api_key)
            .expect("Failed to create HeaderValue from CSFLOAT_API_KEY"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let url = format!(
        "{}?limit={}&sort_by=lowest_price&market_hash_name={}&category=1&category=2",
        base_url, count, market_hash_name
    );
    let listings = client.get(url).send().await?.json::<Vec<Listing>>().await?;

    let total: u64 = listings.iter().map(|l| l.price).sum();
    let average = total / listings.len() as u64;
    Ok(average)
}
