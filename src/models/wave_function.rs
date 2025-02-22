use std::collections::{HashMap, HashSet};
use std::fs;

use image::{self, DynamicImage, GenericImageView};
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use super::direction::Direction;
use super::tile::Tile;

pub struct WaveFunction {
    width: usize,
    height: usize,
    cells: Vec<Vec<HashSet<usize>>>,
    tile_map: HashMap<usize, Tile>,
    socket_map: HashMap<Vec<u32>, usize>,
    rng: ThreadRng,
    socket_id: usize,
    tile_id: usize,
}

impl WaveFunction {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: Vec::with_capacity(height),
            tile_map: HashMap::new(),
            socket_map: HashMap::new(),
            socket_id: 0,
            tile_id: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn print_sockets(&self) {
        println!("Socket mappings:");
        for (tile_id, tile) in &self.tile_map {
            println!(
                "Tile {}: UP={}, RIGHT={}, DOWN={}, LEFT={}",
                tile_id,
                tile.get_socket(0),
                tile.get_socket(1),
                tile.get_socket(2),
                tile.get_socket(3)
            );
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            print!("[");
            for x in 0..self.width {
                let entropy = self.get_entropy(x, y);
                if x == self.width - 1 {
                    print!("{:2}", entropy);
                } else {
                    print!("{:2}, ", entropy);
                }
            }
            println!("]");
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_num_tiles(&self) -> usize {
        self.tile_map.len()
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &HashSet<usize> {
        &self.cells[x][y]
    }

    pub fn get_tile(&self, id: usize) -> &Tile {
        match self.tile_map.get(&id) {
            Some(tile) => tile,
            None => {
                for tile in self.tile_map.values() {
                    println!("{}", tile.get_id());
                }
                panic!("ERROR {}", id);
            }
        }
    }

    pub fn get_max_entropy(&self) -> usize {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.len()).max().unwrap())
            .max()
            .unwrap()
    }

    pub fn get_random_uncollapsed_cell(&self, rng: &mut ThreadRng) -> Option<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.len() > 1)
                    .map(move |(y, _)| (x, y))
            })
            .flatten()
            .choose(rng)
    }

    pub fn get_random_uncollapsed_cell_min_entropy(&self) -> Option<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.len() > 1)
                    .map(move |(y, _)| (x, y))
            })
            .flatten()
            .min_by_key(|(x, y)| self.get_entropy(*x, *y))
    }

    pub fn get_entropy(&self, x: usize, y: usize) -> usize {
        self.cells[x][y].len()
    }

    pub fn collapse_randomly(&mut self, x: usize, y: usize) {
        // Already collapsed
        if self.cells[x][y].len() <= 1 {
            return;
        }

        // Choose a random tile from the available options
        let chosen_tile = self.cells[x][y].iter().choose(&mut self.rng).copied();

        if let Some(tile_id) = chosen_tile {
            // Clear the cell and insert only the chosen tile
            self.cells[x][y].clear();
            self.cells[x][y].insert(tile_id);
        }
    }

    pub fn remove_tile_from_cell(&mut self, x: usize, y: usize, id: usize) {
        self.cells[x][y].remove(&id);
    }

    pub fn init(&mut self, path: String) -> Result<(), std::io::Error> {
        println!("Reading Tiles from fs...");
        for file in fs::read_dir(path)? {
            let path = file?.path();
            let path_string = &path.to_str().unwrap().to_string();
            if fs::metadata(&path)?.is_dir() {
                continue;
            }
            let mut sockets: Vec<usize> = vec![0; 4];
            let img = image::open(path).unwrap();
            sockets[0] = self.get_socket_id(&img, Direction::UP);
            sockets[1] = self.get_socket_id(&img, Direction::RIGHT);
            sockets[2] = self.get_socket_id(&img, Direction::DOWN);
            sockets[3] = self.get_socket_id(&img, Direction::LEFT);
            let tile: Tile = Tile::new(self.tile_id, path_string.to_string(), sockets.clone(), 0);
            self.tile_map.insert(self.tile_id, tile);
            self.tile_id += 1;
            // for i in 0..1 {
            //     let tile: Tile =
            //         Tile::new(self.tile_id, path_string.to_string(), sockets.clone(), i);
            //     self.tile_map.insert(self.tile_id, tile);
            //     self.tile_id += 1;
            // }
        }
        println!("Tile reading done.");
        let tile_id_set: HashSet<usize> = (0..self.tile_id).collect();
        // Generaing the high entropy board
        let row: Vec<HashSet<usize>> = vec![tile_id_set.clone(); self.width];
        for _ in 0..self.height {
            self.cells.push(row.clone());
        }
        println!("Done!");
        let mut test: Vec<&usize> = self.socket_map.values().collect();
        test.sort();
        println!("{:?}", test);
        return Ok(());
    }

    /// For a given tile and a given direction, generate the socket id for other tiles to match it
    /// with.
    /// Currently, the sockets are determined by the pixel values of the edge of the image.
    pub fn get_socket_id(&mut self, img: &DynamicImage, direction: Direction) -> usize {
        let mut pixel_vec: Vec<u32>;
        let pixel_count = match direction {
            Direction::RIGHT | Direction::LEFT => img.height(),
            Direction::UP | Direction::DOWN => img.width(),
        };

        pixel_vec = vec![0; pixel_count as usize];

        for i in 0..pixel_count {
            let pixel = match direction {
                Direction::UP => img.get_pixel(i, 0),
                Direction::DOWN => img.get_pixel(i, pixel_count - 1),
                Direction::LEFT => img.get_pixel(0, i),
                Direction::RIGHT => img.get_pixel(pixel_count - 1, i),
            };
            // Treat R G B as a unique identifier for that socket
            for pow in 0..3 {
                pixel_vec[i as usize] += (pixel.0[pow] as u32) * 1000_u32.pow(pow as u32);
            }
        }
        match self.socket_map.get(&pixel_vec) {
            Some(some) => *some,
            None => {
                self.socket_map.insert(pixel_vec, self.socket_id);
                self.socket_id += 1;
                return self.socket_id - 1;
            }
        }
    }
}
