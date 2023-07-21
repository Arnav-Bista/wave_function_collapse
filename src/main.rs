use image::GenericImageView;
mod models;
mod wfc;
use std::env;

use crate::{models::board::Board, wfc::WFC};

use nannou::prelude::*;

pub fn main() {
    nannou::app(model)
        .view(view)
        .run();
}


struct Model {
    screen_x: usize,
    scren_y: usize,
    board: Board
}

fn model(_app: &App) -> Model {
    let size = (500,500);
    let mut board = Board::new(size.0, size.1);
    board.init("assets/images/".to_string());
    let mut wfc = WFC::new(board);
    wfc.run();
    let board = wfc.get_board();

    println!("MODEl");

    _app.new_window()
        .size(size.0 as u32,size.1 as u32)
        .build()
        .unwrap();

    Model {
        screen_x: size.0,
        scren_y: size.1,
        board
    }
}

fn view(app: &App, model: &Model, frame: Frame) { 
    println!("DRAWING");
    let draw = app.draw();
    let size = (500, 500);
    let grid = (5, 5);
    let tile_size = (size.0 as f32 / grid.0 as f32, size.1 as f32 / grid.1 as f32);
    let win = app.window_rect();
    let top_left: Point2 = win.top_left();

    draw.background().color(WHITESMOKE);

    for i in 0..grid.0 {
        for j in 0..grid.1 {
            let r = Rect::from_x_y_w_h(top_left.x + tile_size.0 * i as f32, top_left.y - tile_size.1 * j as f32, tile_size.0, tile_size.1);

            let tile_id: usize = *model.board.get(i, j).into_iter().next().unwrap();
            let tile = model.board.get_tile(tile_id);
            let texture = wgpu::Texture::from_path(app, tile.get_src()).unwrap();
            
            draw.texture(&texture)
                .x_y(r.x() + r.w() / 2.0, r.y() - r.h() / 2.0) // Center the texture on the rectangle
                .w_h(r.w(), r.h()); // Ensure that the texture covers the entire rectangle
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
