use std::collections::{HashMap, HashSet};

use rand::prelude::*;

use crate::models::{self, tile::Tile, board::Board, direction::Direction};

pub struct WFC {
    board: Board,
    did_update: Vec<Vec<bool>>,
    entropy: usize,
    rng: ThreadRng
}


impl WFC {
    pub fn new(board: Board) -> Self {
        let did_update = vec![vec![false; board.width()]; board.height()];
        Self {
            board,
            did_update, 
            entropy: 0,
            rng: rand::thread_rng()
        }
    }

    pub fn get_board(self) -> Board {
        self.board
    }

    fn get_start_point(&mut self) -> (usize, usize) {
        println!("CALLED");
        let mut entropy = 0;
        // let mut rng = rand::thread_rng();
        // println!("{}",rng.gen_range(0..self.board.height()));

        // let mut index: (usize, usize) = (
        //     rng.gen_range(0..self.board.width()),
        //     rng.gen_range(0..self.board.height()),
        // );
        let mut index: (usize, usize);
        loop {
            index = 
                (
                    self.rng.gen_range(0..self.board.width()),
                    self.rng.gen_range(0..self.board.height()),
                );
            if self.board.get_entropy(index.0, index.1) != 1 {
                break;
            }
        }

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let current = self.board.get(x,y);
                // if current.len() != 1 && current.len() > self.board.get(index.0, index.1).len() {
                //     index.0 = x;
                //     index.1 = y;
             // }
                if current.len() > entropy {
                    entropy = current.len();
                }
            }
        }

        self.entropy = entropy;
        println!("{}", self.board.get_entropy(index.0, index.1));
        println!("{:?}",index);

        index
    }

    fn remove_randomly(&mut self, x: usize, y: usize) {
        if self.board.get_entropy(x, y) <= 1 {
            return;
        }
        self.board.remove_random(x, y);
    }

    fn reset_did_update(&mut self) {
        for i in 0..self.did_update.len() {
            for j in 0..self.did_update[i].len() {
                self.did_update[i][j] = false;
            }
        }
    }

    fn ripple(&mut self, x: usize, y: usize) {
        self.reset_did_update(); 
        self.propotage(x, y);
    }

    fn propotage(&mut self, x:usize, y: usize) {
        self.make_compatible(x, y, Direction::UP);
        self.make_compatible(x, y, Direction::DOWN);
        self.make_compatible(x, y, Direction::LEFT);
        self.make_compatible(x, y, Direction::RIGHT);

    }

    fn make_compatible(&mut self, x:usize, y: usize, direction: Direction) {
        // Edge
        let result = direction.adjust_coordinate(x, y);
        let new_index = match result {
            Some(index) => index,
            None => {
                return;
            },
        };

        if new_index.1 >= self.board.height() || new_index.0 >=self.board.width() {
            return;
        }

        // [y][x]
        if self.did_update[new_index.1][new_index.0] {
            return;
        }

        let mut hashset: HashSet<usize> = HashSet::new();
        let mut deletion_area: Vec<usize> = Vec::new();

        // Get sockets of original
        for tile_id in self.board.get(x, y) {
            let tile = self.board.get_tile(*tile_id);
            hashset.insert(tile.get_socket(direction.to_num()));
        }

        // Remove invalid tiles
        for tile_id in self.board.get(new_index.0, new_index.1) {
            let tile_id = *tile_id;
            let tile = self.board.get_tile(tile_id);
            let socket = tile.get_socket(direction.get_opposite().to_num());
            if !hashset.contains(&socket) {
                deletion_area.push(tile_id)            
            } 
        }
        for tile_id in deletion_area {
            self.board.remove_tile_from_data(x, y, tile_id);
        }

        // [y][x]
        self.did_update[new_index.1][new_index.0] = true;

        // Pass the effect to other starting from new index
        self.propotage(new_index.0, new_index.1);
    }


    pub fn run(&mut self) {
        while self.entropy != 2 {
        // for _ in 0..10000 {
            let index = self.get_start_point();
            self.remove_randomly(index.0, index.1);
            self.ripple(index.0, index.1);
            println!("ENTROPY {}", self.entropy);
        }
        println!("DONE");
        // dbg!(self.board.get_data());
    }


}
