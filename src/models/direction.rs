pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Direction {
    pub fn adjust_coordinate(&self, x: usize, y:usize) -> Option<(usize, usize)> {
        let mut index = (x,y);
        match self {
            Self::UP => {
                if y == 0 {
                    return None;
                }
                index.1 -= 1;
            }
            Self::DOWN => index.1 += 1,
            Self::LEFT => {
                if x == 0 {
                    return None;
                }
                index.0 -= 1;
            }
            Self::RIGHT => index.0 += 1
        };
        Some(index) 
    }

    pub fn to_num(&self) -> usize {
        match self {
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 2,
            Self::LEFT => 3
        }
    }

    pub fn get_opposite(&self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
            Self::DOWN => Self::UP,
            Self::LEFT => Direction::RIGHT
        }
    }
}
