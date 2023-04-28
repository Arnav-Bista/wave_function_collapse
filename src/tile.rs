use serde::Deserialize;
use serde_json::Result;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Tile {
    name: String,
    socket_id: u8,
    sockets: Vec<u8>,
    prototypes: Vec<Prototype>
}

#[derive(Deserialize, Debug)]
struct Prototype {
    rotation: u8,
    src: String
}

impl Tile {
    pub fn get_tile_list(path_to_dir: &str) -> Result<Vec<Tile>> {
        let mut result : Vec<Tile> = Vec::new();
        let paths = fs::read_dir(path_to_dir).unwrap();
        for path in paths {
            // println!("Name: {}", path.unwrap().path().display())
            let file_json = fs::read_to_string(path.unwrap().path())
            .expect("Could not open file.");
            let tile: Tile = serde_json::from_str(&file_json)?;
            result.push(tile);
        }
        Ok(result)
    }
}
