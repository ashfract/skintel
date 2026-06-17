use crate::data;
use crate::models::Rarity;
use crate::models::Skin;
use std::collections::HashMap;

pub async fn group_skins(skins: Vec<Skin>) -> HashMap<String, HashMap<Rarity, Vec<Skin>>> {
    let mut map: HashMap<String, HashMap<Rarity, Vec<Skin>>> = HashMap::new();
    for skin in skins {
        map.entry(skin.collections.clone())
            .or_default()
            .entry(skin.rarity)
            .or_default()
            .push(skin);
    }
    map
}

fn normalise(float: f64, min: f64, max: f64) -> f64 {
    (float - min) / (max - min)
}

fn outcome_float(avg_normalised: f64, outcome_min: f64, outcome_max: f64) -> f64 {
    outcome_min + (outcome_max - outcome_min) * avg_normalised
}

fn max_avg_normalised(target_float: f64, outcome_min: f64, outcome_max: f64) -> f64 {
    (target_float - outcome_min) / (outcome_max - outcome_min).clamp(0.0, 1.0)
}

fn get_valid_targets(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    input_rarity: &Rarity,
    output_rarity: &Rarity,
) -> Vec<Skin> {
    let mut candidates = Vec::new();

    for (_, rarities) in collections {
        let inputs = match rarities.get(input_rarity) {
            Some(s) => s,
            None => continue,
        };
        let outputs = match rarities.get(output_rarity) {
            Some(s) => s,
            None => continue,
        };
        candidates.extend(outputs.iter().cloned());
    }
    candidates
}

async fn get_profitable_targets(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    candidates: Vec<Skin>,
) -> Result<Vec<Skin>, Box<dyn std::error::Error>> {
    let mut targets: Vec<Skin> = Vec::new();
    for target in candidates {
        let target_price =
            data::market::csfloat::get_price(target.market_hash_name.clone()).await?;
        let input_pool = collections
            .get(&target.collections)
            .and_then(|r| Rarity::previous(&target.rarity).and_then(|prev| r.get(&prev)));
        let input_pool = match input_pool {
            Some(p) => p,
            None => continue,
        };
        if input_pool.is_empty() {
            continue;
        }
        let mut min_input_price = u64::MAX;
        for skin in input_pool {
            let price = data::market::csfloat::get_price(skin.market_hash_name.clone()).await?;
            if price < min_input_price {
                min_input_price = price;
            }
        }
        if target_price > min_input_price * 10 {
            targets.push(target);
        }
    }
    Ok(targets)
}
