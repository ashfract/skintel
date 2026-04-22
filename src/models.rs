use serde::Deserialize;

#[derive(Deserialize)]
pub struct Skin {
    pub market_hash_name: String,
    pub rarity: Rarity,
    pub stattrak: bool,
    pub collections: Vec<Collection>,
    pub min_float: f32,
    pub max_float: f32,
    pub lowest_sell: f32,
}

#[derive(Deserialize)]
pub enum Rarity {
    ConsumerGrade,
    IndustrialGrade,
    MilSpec,
    Restricted,
    Classified,
    Covert,
    ExceedinglyRare,
}

#[derive(Deserialize)]
pub struct Collection {
    id: String,
}
