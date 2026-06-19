use crate::models;
use dotenvy::dotenv;
use reqwest;

pub async fn get_bulk_price(
    market_hash_name: String,
    count: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
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
    let encoded_name = urlencoding::encode(&market_hash_name);
    let url = format!(
        "{}?limit={}&sort_by=lowest_price&market_hash_name={}&category=1&type=buy_now",
        base_url, count, encoded_name
    );
    println!("[DEBUG] URL: {}", url);
    let response = client.get(&url).send().await?;
    println!("[DEBUG] Status: {}", response.status());
    let listings = client
        .get(url)
        .send()
        .await?
        .json::<models::CSFloatResponse>()
        .await?
        .data
        .unwrap_or_default();
    println!(
        "[DEBUG] {} listings found for {}\n",
        listings.len(),
        market_hash_name
    );
    if listings.is_empty() {
        return Ok(0);
    }
    let total: u64 = listings.iter().map(|l| l.price).sum();
    let average = total / listings.len() as u64;
    Ok(average)
}

// Get specific listings function to do -> name, max float, amount
//
pub async fn get_specific_listings(
    paint_index: i32,
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
        "{}?limit={}&sort_by=lowest_price&paint_index={}&category=1&max_float={}&type=buy_now",
        base_url, count, paint_index, max_float
    );
    let listings: Vec<models::Listing> = client
        .get(url)
        .send()
        .await?
        .json::<models::CSFloatResponse>()
        .await?
        .data
        .unwrap_or_default();
    println!(
        "[CSFLOAT] Successfully requested data paint_index: {}",
        paint_index
    );
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
            min_float: None,
            max_float: None,
        };
        inputs.push(input);
    }
    Ok(inputs)
}

pub async fn get_price(market_hash_name: String) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(get_bulk_price(market_hash_name, 1).await?)
}
