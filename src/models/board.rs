use std::collections::{hash_map, HashMap, HashSet};
use std::fs;

use image::{self, DynamicImage, GenericImageView};

use super::direction::{Direction, self};
use super::tile::Tile;



pub struct Board {
    width: usize,
    height: usize,
    data: Vec<Vec<HashSet<usize>>>,
    tiles: HashMap<usize, Tile>,
    socket_hash: HashMap<Vec<u32>, usize>,
    socket_id: usize,
    tile_id: usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::with_capacity(height),
            tiles: HashMap::new(),
            socket_hash: HashMap::new(),
            socket_id: 0,
            tile_id: 0,
        }
    }

    pub fn init(&mut self, path: String) {
        for file in fs::read_dir(path).unwrap() {
            let mut sockets: Vec<usize> = vec![0;4];
            let img = image::open(file.unwrap().path()).unwrap();
            sockets[0] = self.get_socket_id(&img, Direction::UP);
            sockets[1] = self.get_socket_id(&img, Direction::RIGHT);
            sockets[2] = self.get_socket_id(&img, Direction::DOWN);
            sockets[3] = self.get_socket_id(&img, Direction::LEFT);
            for i in 0..4 {
                let tile: Tile = Tile::new(self.tile_id, sockets.clone(), i);
                self.tiles.insert(self.tile_id, tile);
                self.tile_id += 1;
            }
        }
        println!("Tile reading done.");
        let set: HashSet<usize> = (0..self.tile_id + 1).collect();
        let row: Vec<HashSet<usize>> = vec![set.clone(); self.width];
        for i in 0..self.height {
            self.data.push(row.clone());
        }
        println!("Done!");
        let mut test: Vec<&usize> = self.socket_hash.values().collect();
        test.sort();
        println!("{:?}", test);
    }

    pub fn get_socket_id(&mut self, img: &DynamicImage, direction: Direction) -> usize {
        let mut pixel_vec: Vec<u32>;
        let dimension = match direction {
            Direction::RIGHT | Direction::LEFT => img.height(),
            Direction::UP | Direction::DOWN => img.width()
        };

        pixel_vec = vec![0;dimension as usize];

        for i in 0..dimension {
            let pixel = match direction {
                Direction::UP => img.get_pixel(i, 0),
                Direction::DOWN => img.get_pixel(i, dimension - 1),
                Direction::LEFT => img.get_pixel(0, i),
                Direction::RIGHT => img.get_pixel(dimension - 1, i)
            };
            for pow in 0..3 {
                pixel_vec[i as usize] += (pixel.0[pow] as u32) * 1000_u32.pow(pow as u32);
            }
        }
        match self.socket_hash.get(&pixel_vec) {
            Some(some) => *some,
            None => {
                self.socket_hash.insert(pixel_vec, self.socket_id);
                self.socket_id += 1;
                return self.socket_id - 1;
            },
        }


    }
}
