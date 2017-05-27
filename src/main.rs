extern crate sdl2;

mod engine;
mod screens;

fn main() {
    // We have to declare texture_creator before game, because texture_creator must stay alive longer
    // than the game.
    let mut texture_creator : sdl2::render::TextureCreator<sdl2::video::WindowContext>;
    let mut game = engine::new("GAMEGAMEGAME");

    // SECTION: TEXTURE LOADING
    texture_creator = game.renderer.window.texture_creator();
    // Load all textures here
    // Using game.load_texture()
    game.renderer.load_texture(&mut texture_creator, "owo", "res/owo.png");

    // Add your initial screen to the game
    game.set_screen(screens::main_menu::new());

    game.start();
}
