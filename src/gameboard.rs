use crate::rotationdirection::{RotationDirection, MoveDirection};
use crate::piece::{Piece, PieceType};
use crate::block::Block;

const FIELD_SIZE: usize = 10 * 20;

pub struct GameBoard {
    field: Vec<Block>,
    pub current_piece: Piece,
}

impl GameBoard {
    pub fn new() -> Self {
        GameBoard {
            field: Vec::new(),
            current_piece: GameBoard::choose_random_piece(),
        }
    }

    fn choose_random_piece() -> Piece {
        Piece::new(rand::random())
    }

    pub fn tick(&mut self) {
        self.current_piece.down();
        // check if can clear row
    }

    pub fn rotate_piece(&self, direction: &RotationDirection) {
        self.current_piece.rotate(direction, true);
    }

    pub fn move_piece(&mut self, direction: &MoveDirection) {
        self.current_piece.moves(direction);
    }
}
