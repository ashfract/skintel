use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    ConsumerGrade,
    IndustrialGrade,
    MilSpec,
    Restricted,
    Classified,
    Covert,
}

impl Rarity {
    pub fn from_str(s: &str) -> Option<Rarity> {
        match s {
            "Consumer Grade" => Some(Rarity::ConsumerGrade),
            "Industrial Grade" => Some(Rarity::IndustrialGrade),
            "MilSpec Grade" => Some(Rarity::MilSpec),
            "Restricted" => Some(Rarity::Restricted),
            "Classified" => Some(Rarity::Classified),
            "Covert" => Some(Rarity::Covert),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Skin {
    pub market_hash_name: String,
    pub collection: String,
    pub min_float: f64,
    pub max_float: f64,
    pub rarity: Rarity,
    pub collections: String,
}
