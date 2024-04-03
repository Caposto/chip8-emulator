// TODO: Create a struct that contains all of the CHIP8 Emulator's specifications
struct chip8 {
    memory: [u8; 4096], // 4 kb of RAM = 4096 bytes. The byte "memory address" will be the index in the array
    registers: [u8; 16],
    keyboard: [u8; 16], // 16 key hexadecimal keyboard
    // display: bool, // TODO: should this be in the sdl project?
    program_counter: u16, // points at instruction in memory
    index_register: u16, // 4kb = 4000 bytes which means 4000 memory addresses. 16 bits covers up to 65,536 different numbers
    stack: bool,
    delay_timer: u8,
    sound_timer: u8
}



pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 4);
    }
}
