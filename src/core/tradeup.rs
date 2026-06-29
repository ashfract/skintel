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
        map.entry(skin.collections.clone())
            .or_default()
            .entry(skin.rarity)
            .or_default()
            .push(skin);
    }
    println!("[GROUPING] {} collections produced", map.len());
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
    let mut candidates: Vec<Skin> = collections
        .iter()
        .filter_map(|(name, rarities)| {
            let has_input = rarities.contains_key(input_rarity);
            let has_output = rarities.contains_key(output_rarity);
            if has_input && has_output {
                rarities.get(output_rarity).cloned()
            } else {
                None
            }
        })
        .flatten()
        .collect();

    candidates.sort_by(|a, b| a.name.cmp(&b.name));
    candidates.dedup_by(|a, b| a.name == b.name);
    println!("[TARGETING] {} candidates produced", candidates.len());
    candidates
}

pub async fn get_profitable_targets(
    collections: &HashMap<String, HashMap<Rarity, Vec<Skin>>>,
    candidates: Vec<Skin>,
    price_cache: &mut HashMap<String, u64>,
) -> Result<Vec<(Skin, TradeUpInput)>, Box<dyn std::error::Error>> {
    let mut targets: Vec<(Skin, TradeUpInput)> = Vec::new();
    for target in candidates {
        let target_fn_name = format!("{} (Factory New)", target.name);
        let target_price = match price_cache.get(&target_fn_name) {
            Some(p) => {
                println!("[CACHE] hit: {}", target_fn_name);
                *p
            }
            None => {
                let p = data::market::csfloat::get_price(target_fn_name.clone()).await?;
                price_cache.insert(target_fn_name.clone(), p);
                p
            }
        };
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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

        let max_avg_normalised = max_avg_normalised(0.07, target.min_float, target.max_float);
        let mut inputs: Vec<TradeUpInput> = Vec::new();
        for skin in input_pool {
            let denormalised_max = denormalise(max_avg_normalised, skin.min_float, skin.max_float);
            let listings =
                data::market::csfloat::get_specific_listings(skin.paint_index, denormalised_max, 1)
                    .await;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let sample = match listings {
                Ok(v) => v.into_iter().next(),
                Err(_) => continue,
            };
            let mut sample = match sample {
                Some(s) => s,
                None => continue,
            };
            sample.min_float = Some(skin.min_float);
            sample.max_float = Some(skin.max_float);
            sample.collection = skin.collections.clone();
            sample.rarity = skin.rarity;
            inputs.push(sample);
        }
        inputs.sort_by_key(|i| i.price);
        let cheapest_input = match inputs.into_iter().next() {
            Some(i) => i,
            None => continue,
        };

        println!(
            "[PROFIT CHECK] {} - target: {}, cheapest_input*10: {}",
            target.name,
            target_price,
            cheapest_input.price * 10
        );

        if target_price > cheapest_input.price * 10 {
            targets.push((target, cheapest_input));
        }
    }
    println!(
        "[TARGETING] {} profitable targets identified",
        targets.len()
    );
    Ok(targets)
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
    println!("[TRADEUP] {} unique collections after dedup", deduped.len());

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
            let float_value = outcome_float(avg_normalised, output.min_float, output.max_float);
            let wear = match float_value {
                0.0..0.07 => "Factory New",
                0.07..0.15 => "Minimal Wear",
                0.15..0.38 => "Field-Tested",
                0.38..0.45 => "Well-Worn",
                0.45..1.0 => "Battle-Scarred",
                _ => continue,
            };
            let market_hash_name = format!("{} ({})", output.name, wear);
            let tradeup_output = TradeUpOutput {
                market_hash_name: market_hash_name.clone(),
                float_value,
                price: data::market::csfloat::get_price(market_hash_name).await?,
                rarity: output.rarity,
                collection: output.collections.clone(),
                probability: 0.0,
                min_float: Some(output.min_float),
                max_float: Some(output.max_float),
            };
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
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
            .sum::<f64>()
            * 0.97;

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
    println!("[TRADEUP] {} tradeups constructed", tradeups.len());
    Ok(tradeups)
}

pub async fn process_tradeups(tradeups: Vec<TradeUp>) -> Vec<TradeUp> {
    let mut tradeups = tradeups;
    tradeups.sort_by(|a, b| b.roi.partial_cmp(&a.roi).unwrap());
    tradeups.retain(|t| t.roi > 0.0);
    println!(
        "[PROCESS] {} profitable tradeups after filtering",
        tradeups.len()
    );
    for t in &tradeups {
        println!(" - {} (roi: {:.2}%", t.input.collection, t.roi * 100.0);
    }
    tradeups
}
