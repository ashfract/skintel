use serde::Deserialize;

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
    pub name: String,
    pub collection: String,
    pub rarity: Rarity,
    pub stattrak: bool,
    pub min_float: f32,
    pub max_float: f32,
}
