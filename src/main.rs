use image::GenericImageView;
mod models;
mod wfc;
use std::env;

use crate::{models::board::Board, wfc::WFC};

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    println!("HELLO WORLD");
    let img = image::open("assets/images/centre-object.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("color {:?}", img.color());
    let mut board = Board::new(10, 10);
    board.init("assets/images/".to_string());
    let mut wfc: WFC = WFC::new(board);
    wfc.run();
    let mut board = wfc.get_board();
    println!("{:?}", board.get_data());
}
