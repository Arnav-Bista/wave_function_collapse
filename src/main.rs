use image::GenericImageView;
use wgpu::sampler_filtering;
mod models;
mod wfc;
use std::env;

use crate::{models::wave_function::WaveFunction, wfc::WFC};

use nannou::prelude::*;

pub fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    screen_x: usize,
    screen_y: usize,
    wfc: WFC,
    textures: Vec<wgpu::Texture>,
}

fn model(_app: &App) -> Model {
    let size = (50, 50);
    let mut wave_function = WaveFunction::new(size.0, size.1);
    wave_function.init("assets/images/".to_string()).unwrap();
    let mut wfc = WFC::new(wave_function);
    wfc.run();
    println!("MODEL");

    _app.new_window()
        .size(500, 500) // Make window bigger for better visibility
        .build()
        .unwrap();

    // Pre-load all textures
    let wave_function = wfc.get_wave_function_ref();
    let textures = (0..wave_function.get_num_tiles())
        .map(|i| {
            let tile = wave_function.get_tile(i);
            wgpu::Texture::from_path(_app, tile.get_src()).unwrap()
        })
        .collect();

    Model {
        screen_x: size.0,
        screen_y: size.1,
        wfc,
        textures,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // if _app.keys.down.contains(&Key::Space) {
    //     model.wfc.step();
    //
    //     // Print sockets of all collapsed cells
    //     let wave_function = model.wfc.get_wave_function_ref();
    //     println!("\nCollapsed Cells Sockets:");
    //     for j in 0..model.screen_y {  // j is row
    //         for i in 0..model.screen_x {  // i is column
    //             let cell = wave_function.get_cell(j, i);  // Swapped to (row, column)
    //             if cell.len() == 1 {
    //                 let tile_id = *cell.iter().next().unwrap();
    //                 let tile = wave_function.get_tile(tile_id);
    //                 println!(
    //                     "Cell ({}, {}) - Tile {}: [N: {}, E: {}, S: {}, W: {}]",
    //                     j, i,  // Print as (row, column)
    //                     tile_id,
    //                     tile.get_socket(0), // North
    //                     tile.get_socket(1), // East
    //                     tile.get_socket(2), // South
    //                     tile.get_socket(3)  // West
    //                 );
    //             }
    //         }
    //     }
    //     println!("------------------------");
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let size = (500, 500);
    let grid = (50, 50);
    let tile_size = (size.0 as f32 / grid.0 as f32, size.1 as f32 / grid.1 as f32);
    let win = app.window_rect();
    let top_left: Point2 = win.top_left();

    draw.background().color(WHITESMOKE);

    let wave_function = model.wfc.get_wave_function_ref();

    for j in 0..grid.1 {
        // j is row
        for i in 0..grid.0 {
            // i is column
            let r = Rect::from_x_y_w_h(
                top_left.x + tile_size.0 * i as f32,
                top_left.y - tile_size.1 * j as f32,
                tile_size.0,
                tile_size.1,
            );

            let cell = wave_function.get_cell(j, i); // Swapped to (row, column)

            // Only draw if the cell is collapsed (has exactly one possibility)
            if cell.len() == 1 {
                let tile_id = *cell.iter().next().unwrap();
                draw.texture(&model.textures[tile_id])
                    .x_y(r.x() + r.w() / 2.0, r.y() - r.h() / 2.0)
                    .w_h(r.w(), r.h());
            } else {
                // Draw a grey rectangle for uncollapsed cells
                draw.rect()
                    .xy(pt2(r.x() + r.w() / 2.0, r.y() - r.h() / 2.0))
                    .w_h(r.w(), r.h())
                    .color(GRAY);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
