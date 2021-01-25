use std::fs::File;
use std::io::prelude::*;

pub struct Chip8 {
    pub memory: Vec<u8>,
    pub v: Vec<u8>,
    pub i: u8,
    pub pc: u8,
    pub stack: Vec<u8>,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let memory = Vec::with_capacity(4096);
        let v = Vec::with_capacity(16);
        let stack = Vec::new();

        return Chip8 {
            memory,
            v,
            i: 0x00,
            pc: 0x00,
            stack,
        };
    }

    fn load_sprites(&mut self) {
        let sprites = [
            0xF0u8, 0x90u8, 0x90u8, 0x90u8, 0xF0u8, // 0
            0x20u8, 0x60u8, 0x20u8, 0x20u8, 0x70u8, // 1
            0xF0u8, 0x10u8, 0xF0u8, 0x80u8, 0xF0u8, // 2
            0xF0u8, 0x10u8, 0xF0u8, 0x10u8, 0xF0u8, // 3
            0x90u8, 0x90u8, 0xF0u8, 0x10u8, 0x10u8, // 4
            0xF0u8, 0x80u8, 0xF0u8, 0x10u8, 0xF0u8, // 5
            0xF0u8, 0x80u8, 0xF0u8, 0x90u8, 0xF0u8, // 6
            0xF0u8, 0x10u8, 0x20u8, 0x40u8, 0x40u8, // 7
            0xF0u8, 0x90u8, 0xF0u8, 0x90u8, 0xF0u8, // 8
            0xF0u8, 0x90u8, 0xF0u8, 0x10u8, 0xF0u8, // 9
            0xF0u8, 0x90u8, 0xF0u8, 0x90u8, 0x90u8, // A
            0xE0u8, 0x90u8, 0xE0u8, 0x90u8, 0xE0u8, // B
            0xF0u8, 0x80u8, 0x80u8, 0x80u8, 0xF0u8, // C
            0xE0u8, 0x90u8, 0x90u8, 0x90u8, 0xE0u8, // D
            0xF0u8, 0x80u8, 0xF0u8, 0x80u8, 0xF0u8, // E
            0xF0u8, 0x80u8, 0xF0u8, 0x80u8, 0x80u8, // F
        ];

        for x in sprites.iter() {
            self.memory.push(*x);
        }
    }

    fn load_program(&mut self) {
        let mut file = File::open("roms/BLITZ").unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data);

        let mut i = 0; 

        for dat in data {
            self.memory.insert(0x200 + i, dat);
            i = i + 1;
        }
    }
}
