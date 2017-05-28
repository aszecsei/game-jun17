# Rust Game
This game uses Rust and SDL2.

## Instructions
This game requires SDL2 to run.

### Windows
1. Go to the [SDL2 download site](https://www.libsdl.org/download-2.0.php).
2. Download the "Visual C++ 32/64-bit" development library.
3. Move `SDL2.dll`, `SDL2.lib`, `SDL2main.lib`, and `SDL2test.lib` from the downloaded archive to your Rust directory. For me, this was `%UserProfile%\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib`.
4. In the command line, use `cargo run` to start the game.
