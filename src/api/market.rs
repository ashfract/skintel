use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub hash_name: String,
    pub sell_price: u32,
    #[serde(rename = "sell_price_text")]
    pub formatted_price: String,
    pub sell_listings: u32,
}

#[derive(Deserialize, Debug)]
struct PageResult {
    results: Vec<MarketData>,
}

async fn fetch_marketdata(start: u32) -> Result<PageResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://steamcommunity.com/market/search/render/?query=&appid=730&start={}&currency=1&norender=1&sort_column=quantity&sort_dir=desc&category_730_Type%5B%5D=tag_CSGO_Type_Pistol&category_730_Type%5B%5D=tag_CSGO_Type_SMG&category_730_Type%5B%5D=tag_CSGO_Type_Rifle&category_730_Type%5B%5D=tag_CSGO_Type_SniperRifle&category_730_Type%5B%5D=tag_CSGO_Type_Shotgun&category_730_Type%5B%5D=tag_CSGO_Type_Machinegun&category_730_Type%5B%5D=tag_CSGO_Type_Knife&category_730_Type%5B%5D=tag_CSGO_Type_Gloves",
        start
    );
    let client = reqwest::Client::new();
    let data = client.get(url).send().await?.json::<PageResult>().await?;
    Ok(data)
}

pub async fn full_fetch() -> Result<Vec<MarketData>, Box<dyn std::error::Error>> {
    let mut skins = Vec::new();
    let mut start = 0;

    loop {
        let page: PageResult = fetch_marketdata(start).await?;
        let before = skins.len();
        skins.extend(
            page.results
                .into_iter()
                .take_while(|item| item.sell_listings >= 100),
        );
        if skins.len() == before || skins.len() - before < 10 {
            break;
        }
        start += 10;
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    }
    Ok(skins)
}
