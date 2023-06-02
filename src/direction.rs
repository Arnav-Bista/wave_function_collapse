pub enum Direction {
    Up,
    Right,
    Down,
    Left
}


impl Direction {
   pub fn get_value(&self) -> u8 {
        match *self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3
        }
    }

    pub fn get_opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Left,
        }
    }

    pub fn get_index_unit_coordinate(&self) -> (i8,i8) {
        match *self {
            Direction::Up => (-1,0),
            Direction::Right => (0,1),
            Direction::Down => (1,0),
            Direction::Left => (0,-1)
        }
    }

}


