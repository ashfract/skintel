use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Rarity {
    #[serde(rename = "Consumer Grade")]
    ConsumerGrade,
    #[serde(rename = "Industrial Grade")]
    IndustrialGrade,
    #[serde(rename = "Mil-Spec Grade")]
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
            "Mil-Spec Grade" => Some(Rarity::MilSpec),
            "Restricted" => Some(Rarity::Restricted),
            "Classified" => Some(Rarity::Classified),
            "Covert" => Some(Rarity::Covert),
            _ => None,
        }
    }
    pub fn previous(&self) -> Option<Rarity> {
        match self {
            Rarity::IndustrialGrade => Some(Rarity::ConsumerGrade),
            Rarity::MilSpec => Some(Rarity::IndustrialGrade),
            Rarity::Restricted => Some(Rarity::MilSpec),
            Rarity::Classified => Some(Rarity::Restricted),
            Rarity::Covert => Some(Rarity::Classified),
            Rarity::ConsumerGrade => None,
        }
    }
    pub fn from_int(i: i32) -> Option<Rarity> {
        match i {
            1 => Some(Rarity::ConsumerGrade),
            2 => Some(Rarity::IndustrialGrade),
            3 => Some(Rarity::MilSpec),
            4 => Some(Rarity::Restricted),
            5 => Some(Rarity::Classified),
            6 => Some(Rarity::Covert),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Skin {
    pub market_hash_name: String,
    pub min_float: f64,
    pub max_float: f64,
    pub rarity: Rarity,
    pub collections: String,
}

// For CSFLOAT API return
#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub float_value: f64,
    pub market_hash_name: String,
    pub collection: String,
    pub rarity: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Listing {
    pub price: u64,
    pub item: Item,
}

#[derive(Deserialize)]
pub struct CSFloatResponse {
    pub data: Option<Vec<Listing>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeUpInput {
    pub market_hash_name: String,
    pub float_value: f64,
    pub price: u64,
    pub rarity: Rarity,
    pub collection: String,
    pub min_float: Option<f64>,
    pub max_float: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TradeUpOutput {
    pub market_hash_name: String,
    pub float_value: f64,
    pub price: u64,
    pub rarity: Rarity,
    pub collection: String,
    pub probability: f64,
    pub min_float: Option<f64>,
    pub max_float: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TradeUp {
    pub input: TradeUpInput,
    pub input_count: u32,
    pub filler: Option<TradeUpInput>,
    pub filler_count: u32,
    pub outputs: Vec<TradeUpOutput>,
    pub total_cost: u64,
    pub worst_value: u64,
    pub best_value: u64,
    pub ev: f64,
    pub roi: f64,
}
