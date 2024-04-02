### Project Structure

As seen with [dhole's](https://dhole.github.io/post/chip8_emu_1/) implementation, the project is broken down into a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) composed of two projects.

1. chip8: Rust library that contains implementation of chip8 and its functions ("backend")
2. sdl: Rust project that imports chip8 functionality ("frontend")

Isolates chip8 code and promotes reuseability.


# Resources

- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.1 
- https://dhole.github.io/post/chip8_emu_1/
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/