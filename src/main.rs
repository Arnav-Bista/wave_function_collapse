use std::env;
use rand::Rng;
mod tile;

const DEFAULT_DIR: &str = "./assets/config/";

pub fn main() {
    // let args: Vec<String> = env::args().collect();
    let data = tile::Tile::get_tile_list(DEFAULT_DIR)
        .expect("Error loading configurations.");
    let board = initialise(data, 10);
    println!("{:#?}",board);

}

fn initialise(data: Vec<tile::Tile>, size: u32) -> Vec<Vec<Vec<tile::Tile>>> {
    let mut board: Vec<Vec<Vec<tile::Tile>>> = Vec::new();

    for i in 0..size {
        board.push(Vec::new());
        for j in 0..size {
            board[i as usize].push(Vec::new());
            for tile in &data {
                let mut tile = tile.clone();
                for k in 0..4 {
                    tile.set_current_rotation(k);
                    board[i as usize][j as usize].push(tile.clone());
                }
            }
        }
    }

    board
}
