mod core;
mod data;
mod models;
use std::env;
use tokio;

use crate::core::tradeup::{
    construct_tradeups, fetch_inputs, get_profitable_targets, get_valid_targets, group_skins,
};
use crate::models::Rarity;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: Vec<String> = env::args().collect();

    let metadata = data::bymekel::get_skins().await?; // works
    let collections = group_skins(metadata).await; // works
    let mut candidates = get_valid_targets(&collections, &Rarity::MilSpec, &Rarity::Restricted);
    candidates.truncate(5);
    let profitable = get_profitable_targets(&collections, candidates).await?;
    let inputs = fetch_inputs(profitable, &collections).await?;
    let tradeups = construct_tradeups(&collections, inputs).await?;

    println!("{:?}", tradeups);

    Ok(())
}
