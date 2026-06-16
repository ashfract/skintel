use serde::Deserialize;

use crate::domain::models::Rarity::ExceedinglyRare;

#[derive(Deserialize, Debug)]
pub enum Rarity {
    ConsumerGrade,
    IndustrialGrade,
    MilSpec,
    Restricted,
    Classified,
    Covert,
    ExceedinglyRare,
}

impl Rarity {
    fn from_str(s: &str) -> Option<Rarity> {
        match s {
            "Consumer Grade" => Some(Rarity::ConsumerGrade),
            "Industrial Grade" => Some(Rarity::IndustrialGrade),
            "Mil Spec" => Some(Rarity::MilSpec),
            "Restricted" => Some(Rarity::Restricted),
            "Classified" => Some(Rarity::Classified),
            "Covert" => Some(Rarity::Covert),
            "Exceedingly Rare" => Some(Rarity::ExceedinglyRare),
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Condition {
    FactoryNew,
    MinimalWear,
    FieldTested,
    WellWorn,
    BattleScarred,
}

#[derive(Deserialize, Debug)]
pub struct Skin {
    pub hash_name: String,
    pub collection: Option<String>,
    pub rarity: Rarity,
    pub stattrak: bool,
    pub min_float: f32,
    pub max_float: f32,
    pub sell_price: u32,
    pub formatted_price: String,
    pub sell_listings: u32,
}
