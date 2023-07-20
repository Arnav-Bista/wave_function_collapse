

pub struct Tile{
    id: usize,
    sockets: Vec<usize>,
    rotation: usize,
}

impl Tile {
    pub fn new(id: usize, sockets: Vec<usize>, rotation: usize) -> Self {
        Self {
            id,
            sockets,
            rotation
        }
    }
}
