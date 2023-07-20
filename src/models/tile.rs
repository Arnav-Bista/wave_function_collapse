
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

    pub fn get_socket(&self, index: usize) -> usize {
        self.sockets[(index + self.rotation) % self.sockets.len()]
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
