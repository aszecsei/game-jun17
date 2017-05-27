extern crate sdl2;

mod engine;
mod screens;

fn main() {
    let mut game = engine::new("GAMEGAMEGAME");

    // SECTION: TEXTURE LOADING
    let texture_creator = game.renderer.texture_creator();
    // Load all textures here
    // Using game.load_texture()

    // Add your initial screen to the game
    game.set_screen(screens::main_menu::new());

    game.start();
}
