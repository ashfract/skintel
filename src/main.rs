mod core;
mod data;
mod models;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let _args: Vec<String> = env::args().collect();
    let skins = data::bymekel::get_skins().await.expect("ByMekel failed");
    println!("{:?}", skins[0]);

    let demo = core::tradeup::group_skins(skins);
    println!("\n{:?}", demo.await.get("The Prisma Collection"));
}
