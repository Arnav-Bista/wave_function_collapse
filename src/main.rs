use std::env;
mod tile;

const DEFAULT_DIR: &str = "./";

pub fn main() {
    // let args: Vec<String> = env::args().collect();

    let data = tile::Tile::get_tile_list(DEFAULT_DIR);
    println!("{:#?}",data);
}
