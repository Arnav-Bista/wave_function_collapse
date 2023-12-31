use std::collections::{hash_map, HashMap, HashSet};
use std::fs;

use image::{self, DynamicImage, GenericImageView};
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use super::direction::Direction;
use super::tile::Tile;



pub struct Board {
    width: usize,
    height: usize,
    data: Vec<Vec<HashSet<usize>>>,
    tiles: HashMap<usize, Tile>,
    socket_hash: HashMap<Vec<u32>, usize>,
    socket_id: usize,
    tile_id: usize,
    rng: ThreadRng
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
            rng: rand::thread_rng()
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_data(&mut self) -> &Vec<Vec<HashSet<usize>>> {
        &self.data
    }

    pub fn get(&self, x: usize, y: usize) -> &HashSet<usize> {
        &self.data[y][x]
    }

    pub fn set(&mut self, x:usize ,y:usize, hashset:HashSet<usize>) {
        self.data[y][x] = hashset;
    }

    pub fn get_tile(&self, id: usize) -> &Tile {
        match self.tiles.get(&id) {
            Some(tile) => tile,
            None => {
                for tile in self.tiles.values() {
                    println!("{}", tile.get_id());
                }
                panic!("ERROR {}",id);
            },
        }
    }

    pub fn get_mut(&mut self, x: usize, y:usize) -> &mut HashSet<usize> {
        &mut self.data[y][x]
    }

    pub fn get_entropy(&self, x:usize, y: usize) -> usize {
        self.data[y][x].len()
    }

    pub fn remove_random(&mut self, x:usize, y:usize) {
        let mut vec: Vec<usize> = self.data[y][x].clone().into_iter().collect();
        let random = self.rng.gen_range(0..self.get_entropy(x, y));
        vec.swap_remove(random);
        self.data[y][x] = vec.into_iter().collect();
    }

    pub fn remove_tile_from_data(&mut self, x:usize, y:usize, id: usize) {
        self.data[y][x].remove(&id);
    }

    pub fn init(&mut self, path: String) {
        for file in fs::read_dir(path).unwrap() {
            let path = file.unwrap().path();
            let path_string = &path.to_str().unwrap().to_string();
            if fs::metadata(&path).unwrap().is_dir() {
                continue;
            }
            let mut sockets: Vec<usize> = vec![0;4];
            let img = image::open(path).unwrap();
            sockets[0] = self.get_socket_id(&img, Direction::UP);
            sockets[1] = self.get_socket_id(&img, Direction::RIGHT);
            sockets[2] = self.get_socket_id(&img, Direction::DOWN);
            sockets[3] = self.get_socket_id(&img, Direction::LEFT);
            for i in 0..4 {
                let tile: Tile = Tile::new(self.tile_id, path_string.to_string() ,sockets.clone(), i);
                self.tiles.insert(self.tile_id, tile);
                self.tile_id += 1;
            }
        }
        println!("Tile reading done.");
        let set: HashSet<usize> = (0..self.tile_id).collect();
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
