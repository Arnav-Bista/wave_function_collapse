use image::GenericImageView;
mod models;
use models::*;

use crate::models::board::Board;

pub fn main() {
    println!("HELLO WORLD");
    let img = image::open("assets/images/centre-object.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("color {:?}", img.color());
    let mut board = Board::new(10, 10);
    board.init("assets/images/".to_string());
}
