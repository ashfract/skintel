use crate::models;
use dotenvy::dotenv;
use reqwest;

pub async fn get_bulk_price(
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
        "{}?limit={}&sort_by=lowest_price&market_hash_name={}&category=1&category=2&type=buy_now",
        base_url, count, market_hash_name
    );
    let listings = client
        .get(url)
        .send()
        .await?
        .json::<Vec<models::Listing>>()
        .await?;

    let total: u64 = listings.iter().map(|l| l.price).sum();
    let average = total / listings.len() as u64;
    Ok(average)
}

// Get specific listings function to do -> name, max float, amount
//
pub async fn get_specific_listings(
    market_hash_name: String,
    max_float: f64,
    count: i64,
) -> Result<Vec<models::TradeUpInput>, Box<dyn std::error::Error>> {
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
        "{}?limit={}&sort_by=lowest_price&market_hash_name={}&category=1&category=2&max_float={}&type=buy_now",
        base_url, count, market_hash_name, max_float
    );
    let listings: Vec<models::Listing> = client
        .get(url)
        .send()
        .await?
        .json::<Vec<models::Listing>>()
        .await?;
    let mut inputs: Vec<models::TradeUpInput> = Vec::new();
    for listing in listings {
        let input = models::TradeUpInput {
            market_hash_name: listing.item.market_hash_name,
            rarity: match models::Rarity::from_int(listing.item.rarity) {
                Some(r) => r,
                None => continue,
            },
            collection: listing.item.collection,
            float_value: listing.item.float_value,
            price: listing.price,
        };
        inputs.push(input);
    }
    Ok(inputs)
}

pub async fn get_price(market_hash_name: String) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(get_bulk_price(market_hash_name, 1).await?)
}
