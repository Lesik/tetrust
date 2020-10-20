use std::time::{Duration, Instant};
use cursive::event::{Event, EventResult};
use cursive::traits::*;
use cursive::XY;
use cursive::Printer;
mod block;
mod gameboard;
mod coordinate;
mod rotationdirection;
mod piece;
mod tile;
mod constants;
use block::Block;
use gameboard::GameBoard;
use rotationdirection::RotationDirection;
use coordinate::Coordinate;
use constants::BOARD_HEIGHT;
use constants::BOARD_SIZE;
use crate::constants::BOARD_WIDTH;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(GameBoardView::new().full_width().full_height());
    // siv.set_fps(20);
    siv.run();
}

struct GameBoardView {
    pixels: [char; BOARD_SIZE],
    game_board: GameBoard,
    last_update: Instant,
}

impl GameBoardView {
    fn new() -> Self {
        GameBoardView {
            pixels: [' '; BOARD_SIZE],
            game_board: GameBoard::new(),
            last_update: Instant::now(),
        }
    }

    fn coordiante_to_pixel(coordinate: &Coordinate) -> usize {
        coordinate.y * BOARD_HEIGHT + coordinate.x
    }

    fn check_tick(&self) {
        if (Instant::now() - self.last_update) > Duration::new(2, 0) {
            // &self.game_board.tick();
            // self.last_update = Instant::now();
        }
    }

    fn render_tiles(&mut self) {
        self.pixels = [' '; BOARD_SIZE];

        for tile in self.game_board.current_piece.tiles.iter() {
            // eprintln!("drawing tile {}:{}", tile.coordinate.x, tile.coordinate.y);
            let pixel = Self::coordiante_to_pixel(&tile.coordinate);
            if pixel < self.pixels.len() {
                self.pixels[pixel] = 'X';
            }
        }
    }
}

impl View for GameBoardView {
    fn draw(&self, printer: &Printer) {
        self.check_tick();

        for (line_no, line) in self.pixels.chunks(BOARD_WIDTH).enumerate() {
            let x: String = line.iter().map(|ch| ch.to_string()).collect();
            // eprintln!("line {} is: {}", line_no, x);
            printer.print_hline(XY::from((0, line_no)), BOARD_WIDTH, x.as_str());
        }

        for (i, pixel) in self.pixels.iter().enumerate() {
            if i % BOARD_WIDTH == 0 {
                // printer.
            }
        }

        //for block in self.game_board.blocks {
        //}
    }

    fn needs_relayout(&self) -> bool {
        return true;
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if let Event::Char(key) = event {
            if key == 'a' {
                self.game_board.tick();
                self.render_tiles();
            }
            if key == 'z' {
                self.game_board.rotate_piece(&RotationDirection::Clockwise);
                // what is the optional callback param for? should we use it?
                return EventResult::Consumed(Option::None);
            }
            else if key == 'x' {
                self.game_board.rotate_piece(&RotationDirection::CounterClockwise);
                return EventResult::Consumed(Option::None);
            }
        }

        EventResult::Consumed(None)
    }
}
