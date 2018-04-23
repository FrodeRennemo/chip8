use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn initialize() {
    let fontset = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    let mut opcode: u16 = 0;
    let mut index_register: u16 = 0;
    let mut program_counter: u16 = 0x200;
    let mut stack_pointer: u8 = 0;
    let mut memory: [u8; 4096];

    // Create a path to the desired file
    let path = Path::new("mem");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    for i in 0..fontset.len() {
        memory[i] = fontset[i];
    }

    let mut registers: [u8; 16];
    let mut keys: [char; 16];
    let mut stack: [u8; 16];
    let mut delay_timer: u8;
    let mut sound_timer: u8;
    let mut graphics: [u8; 64 * 32];
}

fn emulate_cycle() -> () {
    let mut program_counter: u16;
    let mut opcode: u16;
    let mut memory: [u8; 4096];
    // Fetch Opcode
    opcode = memory[program_counter] << 8 | memory[program_counter + 1];
    // Decode Opcode

    // Execute Opcode

    // Update timers
}
