use serde::Deserialize;
use serde_json::Result;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Tile {
    name: String,
    socket_id: u8,
    sockets: Vec<u8>,
    #[serde(skip_deserializing)]
    current_rotation: u8,
    #[serde(skip_deserializing)]
    id: u32,
    src: Vec<String>,
}


impl Tile {
    pub fn get_tile_list(path_to_dir: &str) -> Result<Vec<Tile>> {
        let mut result : Vec<Tile> = Vec::new();
        let paths = fs::read_dir(path_to_dir).unwrap();
        for path in paths {
            // println!("Name: {}", path.unwrap().path().display());
            let file_json = fs::read_to_string(path.as_ref().unwrap().path())
                .expect("Could not open file.");
            let tile: Tile = serde_json::from_str(&file_json)
                .unwrap_or_else(|_| panic!("Error parsing '{}' check if the path is set up properly.", path.unwrap().path().display()));
            for rotation in 0..4 {
                tile.set_current_rotation(rotation);
                result.push(tile.clone());
            }
        }
        Ok(result)
    }

    pub fn set_current_rotation(&mut self, rotation: u8) {
        self.current_rotation = rotation;
    }

    pub fn get_socket(&self, index: u8) -> u8 {
        self.sockets[(index + self.current_rotation) as usize % self.sockets.len()]
    }

    pub fn get_socket_id(&self) -> u8 {
        self.socket_id
    }
}
