use crate::models;
use reqwest;
use serde::Deserialize;

use crate::models::Rarity;

#[derive(Deserialize, Debug)]
pub struct RarityWrapper {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "name")]
    pub market_hash_name: String,
    pub min_float: Option<f32>,
    pub max_float: Option<f32>,
    #[serde(rename = "rarity")]
    pub rarity: RarityWrapper,
    pub stattrak: bool,
}

pub async fn fetch_metadata() -> Result<Vec<models::Skin>, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/ByMykel/CSGO-API/main/public/api/en/skins_not_grouped.json";
    let data: Vec<Metadata> = reqwest::get(url).await?.json::<Vec<Metadata>>().await?;

    let skins = data
        .into_iter()
        .filter_map(|m| {
            Some(models::Skin {
                market_hash_name: m.market_hash_name,
                collection: String::new(),
                min_float: m.min_float? as f64,
                max_float: m.max_float? as f64,
                rarity: Rarity::from_str(&m.rarity.name)?,
                stattrak: m.stattrak,
            })
        })
        .collect();
    println!("Bymykel API fetched successfully");
    Ok(skins)
}
