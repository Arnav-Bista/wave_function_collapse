use std::collections;
use bevy::prelude::*;
use rand::Rng;
mod board;
mod tile;
mod direction;
use direction::Direction;

const DEFAULT_DIR: &str = "./assets/config/";
const DEFAULT_IMAGE_DIR: &str = "./assets/images/";

pub fn main() {
    // let args: Vec<String> = env::args().collect();
    let tiles = tile::Tile::get_tile_list(DEFAULT_DIR)
        .expect("Error loading configurations.");
    let tiles_len = tiles.len();
    let size = 10;
    let mut data = board::Board::init(size, tiles);
    let mut count = 0;
    while !iterate(&mut data, tiles_len) {
        println!("Iteration {count}");
        count += 1;
    }
    // println!("{:?}",data);
    // dbg!(data);
    App::new()
        .add_plugins(DefaultPlugins)            
        .insert_resource(data)
        .add_startup_system(setup)
        .run();
}

fn iterate(data: &mut board::Board, maximum_entropy: usize) -> bool {
    data.flush();
    let mut completed: bool = true;
    let mut least_entropy: usize = maximum_entropy;
    let mut highest_entropy: usize = 0; 
    let mut least_index: (u32, u32) = (0,0);
    let mut rng = rand::thread_rng();

    for i in 0..data.size() {
        for j in 0..data.size() {
            let entropy = data.get(i,j).len();
            // dbg!(data.get(i,j));
            if entropy != 1 {
                completed = false;
            }
            if entropy != 1 && entropy < least_entropy {
                least_entropy = entropy;
                least_index = (i,j);
            }
            if entropy > highest_entropy {
                highest_entropy = entropy;
            }
        }
    }

    if completed {
        return true;
    }    
    if highest_entropy != 1 && highest_entropy == least_entropy {
        // Pick a cell by random
        loop {
            least_index = (
                rng.gen_range(0..data.size()),
                rng.gen_range(0..data.size()) 
            );
            if data.get(least_index.0,least_index.1).len() != 1 {
                break;
            }
        }
        least_entropy = data.get(least_index.0,least_index.1).len();
    }
    // Remove a random tile
    // println!("{} {}", least_entropy, data.get(least_index.0,least_index.1).len());
    data.remove(least_index.0, least_index.1, rng.gen_range(0..least_entropy as u32));
    
    data.update(least_index.0, least_index.1);

    // Update all affected cells
    update(data, least_index, Direction::Up);
    update(data, least_index, Direction::Right);
    update(data, least_index, Direction::Down);
    update(data, least_index, Direction::Left);

    completed
}

fn update(data: &mut board::Board, origin_index: (u32,u32), origin_direction: Direction) {
    // println!("Update!");
    // 0 Up 1 Right 2 Down 3 Left
    let mut target_index: (u32,u32) = origin_index;
    // edge cases
    match origin_direction {
        Direction::Up => {
            if origin_index.0 == 0 {
                return;
            }
            target_index.0 -= 1;
        }
        Direction::Right => {
            if origin_index.1 == data.size() - 1 {
                return;
            }
            target_index.1 += 1;
        }
        Direction::Down => {
            if origin_index.0 == data.size() - 1 {
                return;
            }
            target_index.0 += 1;
        }
        Direction::Left => {
            if origin_index.1 == 0 {
                return;
            }
            target_index.1 -= 1;
        }
    }

    if !data.get_update(target_index.0, target_index.1) && make_compatible(data, target_index, origin_index, &origin_direction) {
        // Recurse
        if !matches!(origin_direction,Direction::Up) {
            update(data, target_index, Direction::Up);
        }
        if !matches!(origin_direction,Direction::Right){
            update(data, target_index, Direction::Right);
        }
        if !matches!(origin_direction,Direction::Down) {
            update(data, target_index, Direction::Down);
        }
        if !matches!(origin_direction,Direction::Left) {
            update(data, target_index, Direction::Left);
        }
    }

}

fn make_compatible(data: &mut board::Board, target_index: (u32,u32), source_index: (u32, u32), direction_from_source: &Direction) -> bool {
    let mut hashset: collections::HashSet<u8> = collections::HashSet::new();
    let mut changed = false;

    for id in data.get(source_index.0,source_index.1) {
        let tile = data.get_tile(*id);
        hashset.insert(
            tile.get_socket(direction_from_source.get_value()),
        );
    }   
    
    // Faster Deletion but does not preserve order (which is irrelevant) 
    let mut i = 0;
    let mut entropy = data.get(target_index.0,target_index.1).len();
    while i < entropy {
        let id = data.get(target_index.0,target_index.1)[i];
        let tile = data.get_tile(id);
        if !hashset.contains(&tile.get_socket(direction_from_source.get_opposite().get_value())) {
            data.get_mut(target_index.0, target_index.1).swap_remove(i);
            changed = true;
            entropy -= 1;
            continue;
        }
        i += 1;
    }
    data.update(target_index.0, target_index.1);
    changed
}


fn setup(mut commands: Commands, data: Res<board::Board>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let scale = Vec3::new(2.0,2.0,2.0);
    for i in 0..data.size() {
        for j in 0..data.size() {
            // let position = Vec3::new((i as f32) * 10.0, (j as f32) * 10.0, 0.0);
            let transform = Transform::from_xyz(i as f32 * 20.0, j as f32 * 20.0, 1.0).with_scale(scale);
            commands.spawn(SpriteBundle {
                texture: asset_server.load("images/".to_owned() + data.get_tile_from_data(i, j).src()),
                transform,
                ..default()
            });
        }
    }
}
