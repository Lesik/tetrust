use std::ops;


pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        // eprintln!("constructing coordinate: {}:{}", x, y);
        Coordinate {
            x,
            y,
        }
    }

    pub fn clone_and_move(&self, other_x: i32, other_y: i32) -> Self {
        // let newx = self.x.wrapping_add(x as usize);
        // let newy = self.y.wrapping_add(y asy usize);
        // eprintln!("clone_and_move x: {} + {} = {}", self.x, x, newx);
        // eprintln!("clone_and_move y: {} + {} = {}", self.y, y, newy);
        Coordinate {
            x: self.x.wrapping_add(other_x as u8),
            y: self.x.wrapping_add(other_y as u8),
        }
    }
}

impl ops::Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Self> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, other: &'b Self) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn sub(self, other: &'b Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
