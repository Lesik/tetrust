use crate::coordinate::Coordinate;
use crate::rotationdirection::{RotationDirection, MoveDirection};

pub struct Tile {
    pub coordinate: Coordinate,
}

impl Tile {
    pub fn new(coordinate: Coordinate) -> Self {
        Tile {
            coordinate,
        }
    }

    pub fn down(&mut self) {
        // eprintln!("        ok everybody LISTEN! old coords: {}:{}", self.coordinate.x, self.coordinate.y);
        self.coordinate += Coordinate::new(0, 1);
        // eprintln!("        and now new coords: {}:{}", self.coordinate.x, self.coordinate.y);
    }
 
    pub fn rotate(&self, center_coordinate: &Coordinate, direction: &RotationDirection) {
        let relative_coordinate: Coordinate = center_coordinate - &self.coordinate;
    }

    pub fn moves(&mut self, direction: &MoveDirection) {
        match direction {
            MoveDirection::Left => self.coordinate.x -= 1,
            MoveDirection::Right => self.coordinate.x += 1,
        }
    }
}
