extern crate sdl2;

mod engine;
mod screens;

fn main() {
    let mut game = engine::new("GAMEGAMEGAME");

    // Add your initial screen to the game

    game.start();
}
