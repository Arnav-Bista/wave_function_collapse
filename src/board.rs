use std::collections;
use crate::tile;

pub struct Board {
    mappings: collections::HashMap<u32,tile::Tile>,
    data: Vec<Vec<Vec<u32>>>,
    size: u32
}

impl Board {
    pub fn new(size: u32) -> Self {
        let mut board = Self {
            mappings: collections::HashMap::new(),
            data: Vec::new(),
            size
        };
        board
    }

    pub fn init(&mut self, tiles: Vec<tile::Tile>) {
        let mut id: u32 = 0;
        for tile in tiles {
            self.mappings.insert(id, tile);
            id += 1;
        }
        let defaut_vec: Vec<u32> = (0..id).collect();
        for i in 0..id {
            self.data.push(Vec::new());
            for j in 0..id {
                self.data[i as usize].push(Vec::new());
            }
        }
    }

    pub fn get_tile(&self, id: u32) -> &tile::Tile {
        let tile = self.mappings.get(&id);
        let result = match tile {
            Some(data) => data,
            None => panic!("Invalid tile ID"),
        };
        result 
    }

    pub fn get(&self, i: u32, j: u32) -> Vec<u32> {
        self.data[i as usize][j as usize]
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn remove(&mut self, i: u32, j: u32, delete_index: u32) {
        self.data[i as usize][j as usize].remove(delete_index as usize);
    }
}
