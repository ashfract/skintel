mod cache;
mod data;
mod models;

fn main() {
    let skins = data::build_skins();
    println!("{:?}", skins);
}
