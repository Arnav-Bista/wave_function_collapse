#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    /// Given a coordinate and a direction, return the new coordinate after moving in that
    /// direction. 
    ///
    /// The coordinate are in respect to a 2D Array.
    /// x -> row
    /// y -> column
    ///
    /// If the new coordinate is negative, return None.
    pub fn adjust_coordinate(&self, mut x: usize, mut y: usize) -> Option<(usize, usize)> {
        match self {
            Self::UP => {
                if x == 0 {
                    return None;
                }
                x -= 1;
            }
            Self::DOWN => x += 1,
            Self::LEFT => {
                if y == 0 {
                    return None;
                }
                y -= 1;
            }
            Self::RIGHT => y += 1
        };
        Some((x, y))
    }

    pub fn to_num(&self) -> usize {
        match self {
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 2,
            Self::LEFT => 3,
        }
    }

    pub fn get_opposite(&self) -> Self {
        match self {
            Self::UP => Self::DOWN,
            Self::RIGHT => Self::LEFT,
            Self::DOWN => Self::UP,
            Self::LEFT => Direction::RIGHT,
        }
    }
}
