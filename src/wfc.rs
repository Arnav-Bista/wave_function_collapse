use std::collections::{HashSet, VecDeque};

use rand::prelude::*;

use crate::models::{direction::Direction, wave_function::WaveFunction};

pub struct WFC {
    wave_function: WaveFunction,
    did_update: Vec<Vec<bool>>,
    entropy: usize,
    rng: ThreadRng,
}

impl WFC {
    pub fn new(board: WaveFunction) -> Self {
        let did_update = vec![vec![false; board.width()]; board.height()];
        Self {
            wave_function: board,
            did_update,
            entropy: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn get_wave_function_ref(&self) -> &WaveFunction {
        &self.wave_function
    }

    fn get_start_point(&mut self) -> (usize, usize) {
        let mut entropy = 0;

        // Select a random cell that has not collapsed
        let (x, y) = self
            .wave_function
            .get_random_uncollapsed_cell_min_entropy()
            .unwrap();
        (x, y)
    }

    fn collapse_cell(&mut self, x: usize, y: usize) {
        if self.wave_function.get_entropy(x, y) <= 1 {
            return;
        }

        // Only collapse if we have valid options
        let cell = self.wave_function.get_cell(x, y);
        if !cell.is_empty() {
            self.wave_function.collapse_randomly(x, y);
        }
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
        self.propagate(x, y);
        self.entropy = self.wave_function.get_max_entropy();
    }

    fn propagate(&mut self, x: usize, y: usize) {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((x, y));
        let directions = [
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ];
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let (x, y) = current;
            for direction in directions {
                if self.make_compatible(x, y, direction) {
                    if let Some(new_coords) = direction.adjust_coordinate(x, y) {
                        queue.push_back(new_coords);
                    }
                }
            }
        }
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.wave_function.width() && y < self.wave_function.height()
    }

    /// Given a coordinate and a direction, the cell after moving in that direction (if it
    /// exists). Then if there are incompatible tiles in the new cell, remove them.
    ///
    /// Returns True if the cell was updated, False otherwise.
    fn make_compatible(
        &mut self,
        current_x: usize,
        current_y: usize,
        direction: Direction,
    ) -> bool {
        // Get the adjacent cell coordinates
        let new_coords = match direction.adjust_coordinate(current_x, current_y) {
            Some(index) => index,
            None => return false,
        };

        let (x, y) = new_coords;
        if !self.in_bounds(x, y) {
            return false;
        }

        let current_cell = self.wave_function.get_cell(current_x, current_y);
        let mut valid_sockets: HashSet<usize> = HashSet::new();

        // Get all valid sockets from the current cell
        for &tile_id in current_cell {
            let tile = self.wave_function.get_tile(tile_id);
            valid_sockets.insert(tile.get_socket(direction.to_num()));
        }

        let adjacent_cell = self.wave_function.get_cell(x, y);
        let mut to_remove = Vec::new();
        let mut has_changes = false;

        for &tile_id in adjacent_cell {
            let tile = self.wave_function.get_tile(tile_id);
            let socket = tile.get_socket(direction.get_opposite().to_num());

            if !valid_sockets.contains(&socket) {
                to_remove.push(tile_id);
                has_changes = true;
            }
        }

        // If we would remove all tiles, this means we have a contradiction
        if to_remove.len() == adjacent_cell.len() {
            println!("WARNING: Contradiction detected at ({}, {})", x, y);
            println!(
                "All tiles would be removed, current valid sockets: {:?}",
                valid_sockets
            );
            panic!("CONTRADICTION!");
        }

        // Remove incompatible tiles
        for tile_id in to_remove {
            self.wave_function.remove_tile_from_cell(x, y, tile_id);
        }

        has_changes
    }

    pub fn run(&mut self) {
        while let Some(_) = self
            .wave_function
            .get_random_uncollapsed_cell(&mut self.rng)
        {
            println!("LOOP");
            // for _ in 0..10000 {
            let (x, y) = self.get_start_point();
            self.collapse_cell(x, y);
            self.ripple(x, y);
            println!("ENTROPY {}", self.entropy);
            self.get_wave_function_ref().print();
        }
        println!("DONE");
        // dbg!(self.board.get_data());
    }

    pub fn step(&mut self) {
        let (x, y) = self.get_start_point();
        self.collapse_cell(x, y);
        self.ripple(x, y);
    }
}
