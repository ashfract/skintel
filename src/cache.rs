use crate::api::market;
use crate::api::metadata;
use crate::domain::models;
use crate::domain::models::Rarity;
use std::collections::HashMap;

//
async fn construct_skins() -> Result<Vec<models::Skin>, Box<dyn std::error::Error>> {
    let market_data = market::full_fetch().await?;
    let meta_data = metadata::fetch_metadata().await?;

    let meta_map: HashMap<String, metadata::Metadata> = meta_data
        .into_iter()
        .map(|m| (m.hash_name.clone(), m))
        .collect();
    let skins: Vec<models::Skin> = market_data
        .into_iter()
        .filter_map(|mk| {
            let meta = meta_map.get(&mk.hash_name)?;
            Some(models::Skin {
                hash_name: mk.hash_name,
                collection: None,
                rarity: Rarity::from_str(&meta.rarity.name)?,
                stattrak: meta.stattrak,
                min_float: meta.min_float?,
                max_float: meta.max_float?,
                sell_price: mk.sell_price,
                formatted_price: mk.formatted_price,
                sell_listings: mk.sell_listings,
            })
        })
        .collect();
    Ok(skins)
}
