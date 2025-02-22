pub struct Tile {
    id: usize,
    src: String,
    sockets: Vec<usize>,
    rotation: usize,
}

impl Tile {
    pub fn new(id: usize, src: String, sockets: Vec<usize>, rotation: usize) -> Self {
        Self {
            id,
            src,
            sockets,
            rotation,
        }
    }

    pub fn get_socket(&self, index: usize) -> usize {
        self.sockets[(index + self.rotation) % self.sockets.len()]
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_src(&self) -> String {
        self.src.to_string()
    }
}
