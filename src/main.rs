use configs::{WINDOW_HEIGHT, WINDOW_WIDTH};
use gamestate::GameState;
use tetra::ContextBuilder;

mod configs;
mod entity;
mod gamestate;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
