use std::collections;
use bevy::prelude::*;
use rand::Rng;
mod board;
mod tile;

const DEFAULT_DIR: &str = "./assets/config/";
const DEFAULT_IMAGE_DIR: &str = "./assets/images/";

pub fn main() {
    // let args: Vec<String> = env::args().collect();
    let tiles = tile::Tile::get_tile_list(DEFAULT_DIR)
        .expect("Error loading configurations.");
    let size = 10;
    let data = board::Board::new(size);
    data.init(tiles);
    let mut count = 0;
    while !iterate(&mut data, 32) {
        println!("Iteration {count}");
        count += 1;
    }
    // println!("{:#?}",data[0][0]);
    // App::new()
    //     .add_plugin(DefaultPlugins)
    //     .add_startup_system(setup)
        // .run();
}

fn iterate(data: board::Board, maximum_entropy: usize) -> bool {
    let mut completed: bool = true;
    let mut least_entropy: usize = maximum_entropy;
    let mut highest_entropy: usize = 0; 
    let mut least_index: (u32, u32) = (0,0);
    let mut rng = rand::thread_rng();

    for i in 0..data.size() {
        for j in 0..data.size() {
            let entropy = data.get(i,j).len();
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
    println!("{} {}", least_entropy, data.get(least_index.0,least_index.1).len());
    data.remove(least_index.0, least_index.1, rng.gen_range(0..least_entropy as u32));

    // Update all affected cells
    update(&mut data, least_index, 0);
    update(&mut data, least_index, 1);
    update(&mut data, least_index, 2);
    update(&mut data, least_index, 3);

    completed
}

fn update(data: &mut board::Board, origin_index: (u32,u32), origin_direction: u8) {
    println!("Update!");
    // 0 Up 1 Right 2 Down 3 Left
    let mut target_index: (u32,u32) = origin_index;
    // edge cases
    match origin_direction {
        // Up
        0 => {
            if origin_index.0 == 0 {
                return;
            }
            target_index.0 -= 1;
        }
        // Right
        1 => {
            if origin_index.1 == data.size() - 1 {
                return;
            }
            target_index.1 += 1;
        }
        // Down
        2 => {
            if origin_index.0 == data.size() - 1 {
                return;
            }
            target_index.0 += 1;
        }
        // Left
        3 => {
            if origin_index.1 == 0 {
                return;
            }
            target_index.1 -= 1;
        }
        _ => panic!("Invalid origin_direciton value")
    }
    
    if make_compatible(data, target_index, origin_index, origin_direction) {
        // Recurse
        if origin_direction != 0 {
            update(data, target_index, 0);
        }
        if origin_direction != 1 {
            update(data, target_index, 1);
        }
        if origin_direction != 2 {
            update(data, target_index, 2);
        }
        if origin_direction != 3 {
            update(data, target_index, 3);
        }
    }

}

fn make_compatible(data: &mut board::Board, target_index: (u32,u32), source_index: (u32, u32), direction_from_source: u8) -> bool {
    if data.get(target_index.0,target_index.1).len() == 1 {
        if data.get(source_index.0,source_index.1).len() == 1 {
            return false;
        }
        return make_compatible(data, source_index, target_index, direction_from_source);
    }
    // 0 Up 1 Right 2 Down 3 Left
    let mut hashmap: collections::HashMap<u8,u8> = collections::HashMap::new();
    let mut changed = false;

    for tile in &data(source_index.0,source_index.1) {
        hashmap.insert(
            tile.get_socket(direction_from_source),
            1 + hashmap.get(&tile.get_socket(direction_from_source)).unwrap_or(&0)
        );
    }   
    // One liner magic (it's cursed)
    // data[target_index.0][target_index.1].retain(|i| {!(*hashmap.get(&i.get_socket_id()).unwrap_or(&0) == 0 && {changed = true; true})});
    // Sadly, it's inefficient

    // Faster Deletion but does not preserve order (which is irrelevant) 
    let mut i = 0;
    let target = &mut data[target_index.0][target_index.1];
    while i < target.len() {
        if *hashmap.get(&target[i].get_socket_id()).unwrap_or(&0) == 0 {
            target.swap_remove(i);
        }
        i += 1;
    }

    changed
}

fn setup(mut commands: Commands) {
    commands.spawn_window(WindowDescriptor {
        title: "My Window".into(),
        width: 800.0,
        height: 600.0,
        resizable: true,
    });
}

