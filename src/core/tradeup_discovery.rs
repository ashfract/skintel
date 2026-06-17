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
