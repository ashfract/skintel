use crate::models::{self, Rarity};
use serde::{self, Deserialize};
use serde_json;
use ureq;

#[derive(Deserialize)]
struct market_skin {
    market_hash_name: String,
    #[serde(rename = "lowestSell")]
    lowest_sell: f32,
}

#[derive(Deserialize)]
struct api_skin {
    market_hash_name: String,
    collections: Vec<models::Collection>,
    rarity: models::Rarity,
    stattrak: bool,
    min_float: f32,
    max_float: f32,
}

fn fetch_metadata() -> Vec<api_skin> {
    let response = ureq::get("https://raw.githubusercontent.com/ByMykel/CSGO-API/main/public/api/en/skins_not_grouped.json")
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    serde_json::from_str(&response).unwrap()
}

fn fetch_marketdata() -> Vec<market_skin> {
    let response = ureq::get("https://api.steamapis.com/v2/steam/items/730/list")
        .header("x-api-key", "A8SmrebZ-JArzFj4djX-BDAc3L4")
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();
    serde_json::from_value(parsed["items"].clone()).unwrap()
}
fn build_skins() -> Vec<models::Skin> {
    let metadata = fetch_metadata();
    let marketdata = fetch_marketdata();

    let market_map: std::collections::HashMap<String, market_skin> = marketdata
        .into_iter()
        .map(|s| (s.market_hash_name.clone(), s))
        .collect();

    metadata
        .into_iter()
        .filter_map(|meta| {
            let price = market_map.get(&meta.market_hash_name)?;
            Some(models::Skin {
                market_hash_name: meta.market_hash_name,
                rarity: meta.rarity,
                stattrak: meta.stattrak,
                collections: meta.collections,
                min_float: meta.min_float,
                max_float: meta.max_float,
                lowest_sell: price.lowest_sell,
            })
        })
        .collect()
}
