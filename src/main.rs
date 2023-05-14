use std::env;
use std::collections;
use bevy::utils::hashbrown::hash_map;
use rand::Rng;
mod tile;

const DEFAULT_DIR: &str = "./assets/config/";

pub fn main() {
    // let args: Vec<String> = env::args().collect();
    let data = tile::Tile::get_tile_list(DEFAULT_DIR)
        .expect("Error loading configurations.");
    let mut data = initialise(data, 10);
    // println!("{:#?}",board);
    let mut count = 0;
    println!("Begin!");
    while !iterate(&mut data, 4) {
        println!("Iteration {count}");
        count += 1;
    }

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

fn iterate(data: &mut Vec<Vec<Vec<tile::Tile>>>, maximum_entropy: usize) -> bool {
    let mut completed: bool = true;
    let mut least_entropy: usize = maximum_entropy;
    let mut highest_entropy: usize = 0; 
    let mut least_index: (usize, usize) = (0,0);
    let mut rng = rand::thread_rng();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let entropy = data[i][j].len();
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
    
    if highest_entropy == least_entropy {
        // Pick a cell by random
        loop {
            least_index = (
                rng.gen_range(0..data.len()),
                rng.gen_range(0..data[0].len())
            );
            if data[least_index.0][least_index.1].len() != 1 {
                break;
            }
        }
    }
    
    // Remove a random tile
    data[least_index.0][least_index.1].remove(rng.gen_range(0..least_entropy));

    // Update all affected cells
    update(data, least_index, 0);
    update(data, least_index, 1);
    update(data, least_index, 2);
    update(data, least_index, 3);

    completed
}

fn update(data: &mut Vec<Vec<Vec<tile::Tile>>>,origin_index: (usize,usize), origin_direction: u8) {
    println!("Update!");
    // 0 Up 1 Right 2 Down 3 Left
    let mut target_index: (usize,usize) = origin_index;
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
            if origin_index.1 == data[0].len() - 1 {
                return;
            }
            target_index.1 += 1;
        }
        // Down
        2 => {
            if origin_index.0 == data.len() - 1 {
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

fn make_compatible(data: &mut Vec<Vec<Vec<tile::Tile>>>, target_index: (usize,usize), source_index: (usize, usize), direction_from_source: u8) -> bool {
    // 0 Up 1 Right 2 Down 3 Left
    let mut hashmap: collections::HashMap<u8,u8> = collections::HashMap::new();
    let mut changed = false;

    for tile in &data[source_index.0][source_index.1] {
        hashmap.insert(
            tile.get_socket(direction_from_source),
            1 + hashmap.get(&tile.get_socket(direction_from_source)).unwrap_or(&0)
        );
    }

    data[target_index.0][target_index.1].retain(|i| {
        let res = *hashmap.get(&i.get_socket_id()).unwrap_or(&0);
        if res == 0 {
            changed = true;
            return false;
        }
        else {
            return true;
        }
    });

    changed
}
