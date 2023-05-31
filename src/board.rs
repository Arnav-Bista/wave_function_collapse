use std::collections;
use crate::tile;

#[derive(Debug)]
pub struct Board {
    mappings: collections::HashMap<u32,tile::Tile>,
    data: Vec<Vec<Vec<u32>>>,
    size: u32
}

impl Board {
    pub fn init(size: u32, tiles: Vec<tile::Tile>) -> Self {
        let mut board = Self {
            mappings: collections::HashMap::new(),
            data: Vec::new(),
            size
        };
        let mut id: u32 = 0;
        for tile in tiles {
            board.mappings.insert(id, tile);
            id += 1;
        }
        let defaut_vec: Vec<u32> = (0..id).collect();
        for i in 0..id {
            board.data.push(Vec::new());
            for _ in 0..id {
                board.data[i as usize].push(defaut_vec.clone());
            }
        }
        board
    }

    pub fn get_tile(&self, id: u32) -> &tile::Tile {
        let tile = self.mappings.get(&id);
        let result = match tile {
            Some(data) => data,
            None => panic!("Invalid tile ID"),
        };
        result 
    }

    pub fn get_data(&self) -> &Vec<Vec<Vec<u32>>> {
        &self.data
    }

    pub fn get(&self, i: u32, j: u32) -> &Vec<u32> {
        &self.data[i as usize][j as usize]
    }

    pub fn get_mut(&mut self, i: u32, j:u32) -> &mut Vec<u32> {
        &mut self.data[i as usize][j as usize]
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn remove(&mut self, i: u32, j: u32, delete_index: u32) {
        self.data[i as usize][j as usize].remove(delete_index as usize);
    }
}
