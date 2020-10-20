use crate::tile::Tile;
use crate::coordinate::Coordinate;
use crate::constants::BOARD_WIDTH;
use crate::rotationdirection::{RotationDirection, MoveDirection};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum PieceType {
    L,
    I,
    S,
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0, 3) {
            0 => PieceType::L,
            1 => PieceType::I,
            _ => PieceType::S,
        }
    }
}

pub struct Piece {
    piece_color: usize,
    pub tiles: [Tile; 4],
    rotation: usize,    // TODO: transform into enum?
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        let topcenter = Coordinate::new(BOARD_WIDTH / 2, 0);
        let p = Piece {
            piece_color: 0,
            tiles: match piece_type {
                PieceType::L => [
                    Tile::new(topcenter.clone_and_move(0, 1)),
                    Tile::new(topcenter.clone_and_move(-1, 0)),
                    Tile::new(topcenter.clone_and_move(-1, 1)),
                    Tile::new(topcenter.clone_and_move(1, 1)),
                ],
                PieceType::I => [
                    Tile::new(topcenter.clone_and_move(0, 0)),
                    Tile::new(topcenter.clone_and_move(-1, 0)),
                    Tile::new(topcenter.clone_and_move(1, 0)),
                    Tile::new(topcenter.clone_and_move(2, 0)),
                ],
                PieceType::S => [
                    Tile::new(topcenter.clone_and_move(0, 1)),
                    Tile::new(topcenter.clone_and_move(0, 0)),
                    Tile::new(topcenter.clone_and_move(1, 0)),
                    Tile::new(topcenter.clone_and_move(-1, 1)),
                ],
            },
            rotation: 0,
        };
        for tile in p.tiles.iter() {
            // eprintln!("ok guys {}:{}", tile.coordinate.x, tile.coordinate.y);
        }
        return p;
    }

    pub fn down(&mut self) {
        for tile in self.tiles.iter_mut() {
            (*tile).down();
        }
    }

    pub fn rotate(&self, direction: &RotationDirection, needs_offset: bool) {
        //self.rotation += direction;     // need to % (mod) this

        for tile in self.tiles.iter() {
            // give tile nr. 0 as anchor point over which to rotate
            tile.rotate(&self.tiles[0].coordinate, direction);
        }
    }

    pub fn moves(&mut self, direction: &MoveDirection) {
        for tile in self.tiles.iter_mut() {
            tile.moves(direction);
        }
    }
}
