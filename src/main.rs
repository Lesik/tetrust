use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
mod piece;
mod tile;
mod coordinate;
mod block;
mod constants;
mod rotationdirection;
use rotationdirection::RotationDirection;
mod gameboard;
use gameboard::GameBoard;
use std::time::{Duration, Instant};
use crate::rotationdirection::MoveDirection;

struct MainState {
    gameboard: GameBoard,
    last_update: Instant,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            gameboard: GameBoard::new(),
            last_update: Instant::now(),
        };
        Ok(s)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if Instant::now() - self.last_update >= Duration::from_secs(1) {
            self.gameboard.tick();
            self.last_update = Instant::now();
        }
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        for tile in self.gameboard.current_piece.tiles.iter() {
            let rect: graphics::Rect = graphics::Rect{
                x: f32::from(10.0 * tile.coordinate.x as f32),
                y: f32::from(10.0 * tile.coordinate.y as f32),
                w: 10.0,
                h: 10.0,
            };
            eprintln!("{}:{}   {}:{}", rect.x, rect.y, rect.w, rect.h);
            let circle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),));
        }
        graphics::present(ctx)?;
        return Ok(());
        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     na::Point2::new(self.pos_x, 380.0),
        //     100.0,
        //     2.0,
        //     graphics::WHITE,
        // )?;
        // graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            ggez::event::KeyCode::Left => self.gameboard.move_piece(&MoveDirection::Left),
            ggez::event::KeyCode::Right => self.gameboard.move_piece(&MoveDirection::Right),
            ggez::event::KeyCode::Z => self.gameboard.rotate_piece(&RotationDirection::CounterClockwise),
            ggez::event::KeyCode::X => self.gameboard.rotate_piece(&RotationDirection::CounterClockwise),
            _ => (),
        }
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    ggez::event::run(ctx, event_loop, state)
}
