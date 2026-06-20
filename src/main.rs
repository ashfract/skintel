mod core;
mod data;
mod models;
use std::collections::HashMap;
use std::env;
use tokio::{self};

use crate::core::tradeup::{
    construct_tradeups, get_profitable_targets, get_valid_targets, group_skins, process_tradeups,
};
use crate::models::Rarity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: Vec<String> = env::args().collect();

    let mut price_cache: HashMap<String, u64> = HashMap::new();

    let metadata = data::bymekel::get_skins().await?; // works
    let collections = group_skins(metadata).await; // works
    let mut candidates = get_valid_targets(&collections, &Rarity::MilSpec, &Rarity::Restricted);
    candidates.truncate(20);
    let profitable = get_profitable_targets(&collections, candidates, &mut price_cache).await?;
    let tradeups = construct_tradeups(&collections, profitable).await?;
    //let processed_tradeups = process_tradeups(tradeups).await;

    println!("{:?}", tradeups);
    Ok(())
}
