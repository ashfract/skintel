use crate::data;
use crate::models;
use crate::models::Rarity;
use crate::models::Skin;
use crate::models::TradeUp;
use crate::models::TradeUpInput;
use crate::models::TradeUpOutput;
use std::collections::HashMap;

pub async fn group_skins(skins: Vec<Skin>) -> HashMap<String, HashMap<Rarity, Vec<Skin>>> {
    let mut map: HashMap<String, HashMap<Rarity, Vec<Skin>>> = HashMap::new();
    for skin in skins {
        println!(
            "Adding skin: {} to collection: {}",
            skin.market_hash_name, skin.collections
        );
        map.entry(skin.collections.clone())
            .or_default()
            .entry(skin.rarity)
            .or_default()
            .push(skin);
    }
    println!("[DEBUG] Successfully produced collections (grouped)");
    //println!("[DEBUG] {:?}", map);
    map
}

fn normalise(float: f64, min: f64, max: f64) -> f64 {
    (float - min) / (max - min)
}

fn denormalise(normalised: f64, min: f64, max: f64) -> f64 {
    normalised * (max - min) + min
}

fn outcome_float(avg_normalised: f64, outcome_min: f64, outcome_max: f64) -> f64 {
    outcome_min + (outcome_max - outcome_min) * avg_normalised
}

fn max_avg_normalised(target_float: f64, outcome_min: f64, outcome_max: f64) -> f64 {
    (target_float - outcome_min) / (outcome_max - outcome_min).clamp(0.0, 1.0)
}

pub fn get_valid_targets(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    input_rarity: &Rarity,
    output_rarity: &Rarity,
) -> Vec<Skin> {
    println!("DEBUG: Searching for input_rarity: {:?}", input_rarity);

    let mut candidates: Vec<Skin> = collections
        .iter()
        .filter_map(|(name, rarities)| {
            let has_input = rarities.contains_key(input_rarity);
            let has_output = rarities.contains_key(output_rarity);
            println!(
                "Collection: {}, Has Input: {}, Has Output: {}",
                name, has_input, has_output
            );
            println!("[DEBUG] Input rarity: {:?}", input_rarity);

            if has_input && has_output {
                rarities.get(output_rarity).cloned()
            } else {
                None
            }
        })
        .flatten()
        .collect();

    println!("[DEBUG] Successfully produced candidates");
    //println!("{:?}", candidates);
    candidates.sort_by(|a, b| a.market_hash_name.cmp(&b.market_hash_name));
    candidates.dedup_by(|a, b| a.market_hash_name == b.market_hash_name);
    candidates
}

pub async fn get_profitable_targets(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    candidates: Vec<Skin>,
) -> Result<Vec<Skin>, Box<dyn std::error::Error>> {
    let mut targets: Vec<Skin> = Vec::new();
    for target in candidates {
        let target_price =
            data::market::csfloat::get_price(target.market_hash_name.clone()).await?;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
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
            println!(
                "[DEBUG] Checking target: {} -> Input: {}",
                target.market_hash_name, skin.market_hash_name
            );
            let price = data::market::csfloat::get_price(skin.market_hash_name.clone()).await?;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            if price < min_input_price {
                min_input_price = price;
            }
        }
        if target_price > min_input_price * 10 {
            targets.push(target);
        }
    }
    println!("[DEBUG] Profitable targets identified");
    Ok(targets)
}

pub async fn fetch_inputs(
    profitable_targets: Vec<Skin>,
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
) -> Result<Vec<(Skin, models::TradeUpInput)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for target in profitable_targets {
        let max_avg_normalised = max_avg_normalised(0.07, target.min_float, target.max_float);
        let input_rarity = match Rarity::previous(&target.rarity) {
            Some(r) => r,
            None => continue,
        };
        let rarities = match collections.get(&target.collections) {
            Some(r) => r,
            None => continue,
        };
        let input_pool = match rarities.get(&input_rarity) {
            Some(i) => i,
            None => continue,
        };
        let mut inputs: Vec<models::TradeUpInput> = Vec::new();
        for input in input_pool {
            let denormalised_max =
                denormalise(max_avg_normalised, input.min_float, input.max_float);
            let results = data::market::csfloat::get_specific_listings(
                input.market_hash_name.clone(),
                denormalised_max,
                1,
            )
            .await;
            let sample = match results {
                Ok(v) => v.into_iter().next(),
                Err(_) => continue,
            };
            let mut sample = match sample {
                Some(s) => s,
                None => continue,
            };
            sample.min_float = Some(input.min_float);
            sample.max_float = Some(input.max_float);
            inputs.push(sample.clone());
        }
        inputs.sort_by_key(|i| i.price);
        let cheapest_input = match inputs.into_iter().next() {
            Some(i) => i,
            None => continue,
        };

        results.push((target, cheapest_input));
    }
    Ok(results)
}

pub async fn construct_tradeups(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    io_pair: Vec<(Skin, TradeUpInput)>,
) -> Result<Vec<models::TradeUp>, Box<dyn std::error::Error>> {
    let mut tradeups = Vec::new();
    let mut deduped: HashMap<String, (Skin, models::TradeUpInput)> = HashMap::new();
    for (target, input) in io_pair {
        deduped
            .entry(target.collections.clone())
            .and_modify(|e| {
                if input.price < e.1.price {
                    *e = (target.clone(), input.clone());
                }
            })
            .or_insert((target, input));
    }
    //let mut outputs: Vec<TradeUpOutput> = Vec::new();
    for (target, input) in deduped.values() {
        let rarities = match collections.get(&target.collections) {
            Some(r) => r,
            None => continue,
        };
        let output_pool = match rarities.get(&target.rarity) {
            Some(o) => o,
            None => continue,
        };
        let mut tradeup_outputs: Vec<TradeUpOutput> = Vec::new();
        let avg_normalised = normalise(
            input.float_value,
            input.min_float.unwrap_or(0.0),
            input.max_float.unwrap_or(1.0),
        );
        for output in output_pool {
            let tradeup_output = TradeUpOutput {
                market_hash_name: output.market_hash_name.clone(),
                float_value: outcome_float(avg_normalised, output.min_float, output.max_float),
                price: data::market::csfloat::get_price(output.market_hash_name.clone()).await?,
                rarity: output.rarity,
                collection: output.collections.clone(),
                probability: 0.0,
                min_float: Some(output.min_float),
                max_float: Some(output.max_float),
            };
            tradeup_outputs.push(tradeup_output);
        }
        let probability = 1.0 / tradeup_outputs.len() as f64; // PLACEHOLDER UNTIL FILLERS
        for out in &mut tradeup_outputs {
            out.probability = probability;
        }
        let total_cost = input.price * 10; // placeholder
        let worst_value = tradeup_outputs.iter().map(|o| o.price).min().unwrap_or(0);
        let best_value = tradeup_outputs.iter().map(|o| o.price).max().unwrap_or(1);
        let ev = tradeup_outputs
            .iter()
            .map(|o| o.price as f64 * o.probability)
            .sum();

        let trade_up = TradeUp {
            input: input.clone(),
            input_count: 10, // placeholder, need to do ratios later
            filler: None,
            filler_count: 0, // placeholder
            outputs: tradeup_outputs,
            total_cost, // placeholder
            worst_value,
            best_value,
            ev,
            roi: (ev - total_cost as f64) / total_cost as f64,
        };
        tradeups.push(trade_up);
    }
    Ok(tradeups)
}
