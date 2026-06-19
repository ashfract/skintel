use crate::models;
use reqwest;
use serde::Deserialize;

use crate::models::Rarity;

#[derive(Deserialize, Debug)]
pub struct RarityWrapper {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CollectionsWrapper {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    #[serde(deserialize_with = "deserialize_optional_string_to_i32")]
    pub paint_index: Option<i32>,
    pub min_float: Option<f64>,
    pub max_float: Option<f64>,
    #[serde(rename = "rarity")]
    pub rarity: RarityWrapper,
    #[serde(default)]
    pub collections: Vec<CollectionsWrapper>,
}

fn deserialize_optional_string_to_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = serde::Deserialize::deserialize(deserializer)?;
    match opt {
        Some(s) => s.parse::<i32>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub async fn get_skins() -> Result<Vec<models::Skin>, Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/ByMykel/CSGO-API/main/public/api/en/skins.json";
    let data: Vec<Metadata> = reqwest::get(url).await?.json::<Vec<Metadata>>().await?;

    let skins = data
        .into_iter()
        .filter_map(|m| {
            let coll = m.collections.first()?;

            Some(models::Skin {
                name: m.name,
                paint_index: m.paint_index?,
                min_float: m.min_float? as f64,
                max_float: m.max_float? as f64,
                rarity: Rarity::from_str(&m.rarity.name)?,
                collections: coll.name.clone(),
            })
        })
        .collect();
    println!("[BYMEKEL] Metadata fetched successfully");
    Ok(skins)
}
