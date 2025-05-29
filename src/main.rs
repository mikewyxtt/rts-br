mod engine;
mod game;

use macroquad::prelude::*;

#[macroquad::main("arugula")]
async fn main() {
    game::main();
}
