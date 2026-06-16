use crate::domain::models;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RarityWrapper {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub min_float: Option<f32>,
    pub max_float: Option<f32>,
    #[serde(rename = "rarity")]
    pub rarity: RarityWrapper,
    pub stattrak: bool,
}

pub async fn fetch_metadata() -> Result<Vec<Metadata>, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/ByMykel/CSGO-API/main/public/api/en/skins.json";
    let data: Vec<Metadata> = reqwest::get(url).await?.json::<Vec<Metadata>>().await?;
    Ok(data)
}
