use crate::renderer::Renderer;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::{thread, time};

pub struct Cpu {
    pub renderer: Renderer,
    pub memory: Vec<u8>,
    pub delay_timer: u8,
    pub v: Vec<u8>,
    pub i: u16,
    pub pc: u16,
    pub stack: Vec<u16>,
    pub speed: u8,
    pub paused: bool,
}

impl Cpu {
    pub fn new(renderer: Renderer) -> Cpu {
        let memory = vec![0; 4096];
        let v = vec![0; 16];
        let stack = Vec::new();

        return Cpu {
            renderer,
            memory,
            delay_timer: 60,
            v,
            i: 0x00,
            pc: 0x200,
            stack,
            speed: 1,
            paused: false,
        };
    }

    pub fn load_sprites(&mut self) {
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

        let mut i = 0;

        for x in sprites.iter() {
            self.memory[i] = x.clone();
            i = i + 1;
        }
    }

    pub fn load_program(&mut self) {
        let mut file = File::open("roms/BC_test.ch8").unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();

        for (i, byte) in data.iter().enumerate() {
            self.memory.insert(512 + i, byte.clone());
        }

        self.load_sprites();
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn cycle(&mut self) -> u8 {
        for _i in 0..self.speed {
            if !self.paused {
                let higher = (self.memory[self.pc as usize] as u16) << 8;
                let lower = self.memory[self.pc as usize + 1] as u16;

                self.execute_instruction(higher + lower);
            }
        }

        if !self.paused {
            self.update_timers();
        }

        return self.renderer.render();
    }

    fn get_type(&self, opcode: u16) -> &str {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0xFF {
                0xE0 => "Clear",

                0xEE => "Return",

                _ => panic!("Instruction not know: {}", opcode),
            },
            0x1000 => "Jump",

            0x2000 => "Call",

            0x3000 => "Skip if Equals Byte",

            0x4000 => "Skip if not equals",

            0x5000 => "Skip if Equals",

            0x6000 => "Load byte",

            0x7000 => "Add byte",

            0x8000 => match opcode & 0xF {
                0x0 => "Move",

                0x1 => "Or",

                0x2 => "And",

                0x3 => "Xor",

                0x4 => "Add",

                0x5 => "Sub",

                0x6 => "Shift Right",

                0x7 => "Reverse sub",

                0xE => "Shift left",
                _ => panic!("Instruction not know: {}", opcode),
            },

            0x9000 => "Skip if not equal",

            0xA000 => "Load i",

            0xB000 => "Jump Plus Zero",

            0xC000 => "Random",

            0xD000 => "Draw",

            0xE000 => match opcode & 0xFF {
                0x9E => "Skip if pressed",

                0xA1 => "Skip if not pressed",
                _ => panic!("Instruction not know: {}", opcode),
            },

            0xF000 => match opcode & 0xFF {
                0x07 => "Load Delay Timer",
                0x0A => "Wait for key press",
                0x15 => "Set delay timer",
                0x18 => "Set sound timer",
                0x1E => "Add to i",
                0x29 => "Load Sprite",
                0x33 => "BCD Representation",
                0x55 => "Store register",
                0x65 => "Load Register",
                _ => panic!("Instruction not know: {}", opcode),
            },
            _ => panic!("Instruction not know: {}", opcode),
        }
    }

    fn execute_instruction(&mut self, opcode: u16) {
        println!("Type: {:X?} - {:X?}", self.get_type(opcode), opcode);

        let x = opcode >> 8 & 0xF;
        let y = opcode >> 4 & 0xF;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0xFF {
                0xE0 => {
                    self.renderer.clear();
                    self.pc = self.pc + 2;
                }
                0xEE => {
                    self.pc = self.stack.pop().unwrap();
                }
                _ => panic!("Instruction not know: {}", opcode),
            },
            0x1000 => {
                if opcode == 0x130E {
                    let mut stdin = io::stdin();
                    let mut stdout = io::stdout();

                    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
                    write!(stdout, "Press any key to continue...").unwrap();
                    stdout.flush().unwrap();

                    // Read a single byte and discard
                    let _ = stdin.read(&mut [0u8]).unwrap();

                    std::process::exit(0);
                }
                self.pc = opcode & 0xFFF;
            }

            0x2000 => {
                self.stack.push(self.pc);
                self.pc = opcode & 0xFFF;
            }

            0x3000 => {
                if self.v[x as usize] == (opcode as u8 & 0xFF) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x4000 => {
                if self.v[x as usize] != (opcode as u8 & 0xFF) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x6000 => {
                self.v[x as usize] = opcode as u8 & 0xFF;
                self.pc += 2;
            }

            0x7000 => {
                self.v[x as usize] = self.v[x as usize] + opcode as u8 & 0xFF;
                self.pc += 2;
            }

            0x8000 => match opcode & 0xF {
                0x0 => {
                    self.v[x as usize] = self.v[y as usize];
                    self.pc += 2;
                }

                0x1 => {
                    self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                    self.pc += 2;
                }

                0x2 => {
                    self.v[x as usize] &= self.v[y as usize];
                    self.pc += 2;
                }

                0x3 => {
                    self.v[x as usize] ^= self.v[y as usize];
                    self.pc += 2;
                }

                0x4 => {
                    let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;

                    if sum > 0xff {
                        self.v[0xf] = 1;
                    } else {
                        self.v[0xf] = 0;
                    }

                    self.v[x as usize] = sum as u8;
                    self.pc += 2;
                }

                0x5 => {
                    self.v[0xF] = 0;

                    if self.v[x as usize] > self.v[y as usize] {
                        self.v[0xF] = 1;
                    }

                    let value1 = self.v[x as usize];
                    let value2 = self.v[y as usize];

                    self.v[x as usize] = value1.wrapping_sub(value2);
                    self.pc += 2;
                }

                0x6 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;

                    self.v[x as usize] >>= 1;
                    self.pc += 2;
                }

                0x7 => {
                    self.v[0xF] = 0;

                    if self.v[y as usize] > self.v[x as usize] {
                        self.v[0xF] = 1;
                    }

                    self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
                    self.pc += 2;
                }

                0xE => {
                    self.v[0xF] = self.v[x as usize] >> 7;
                    self.v[x as usize] <<= 1;
                    self.pc += 2;
                }
                _ => {
                    panic!("Instruction not know: {}", opcode);
                }
            },

            0x9000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0xA000 => {
                self.i = opcode & 0xFF;
                self.pc += 2;
            }

            0xB000 => {
                self.pc = (opcode & 0xFF) + self.v[0] as u16;
                self.pc += 2;
            }

            0xC000 => {
                let ran: u8 = rand::random();

                self.v[x as usize] = ran & (opcode as u8 & 0xff);
                self.pc += 2;
            }

            0xD000 => {
                println!("drawing");
                let width: u8 = 8;
                let height: u8 = opcode as u8 & 0xf;
                
                self.v[0xF] = 0;

                for row in 0..height {
                    let index = self.i + row as u16;
                    let mut sprite = self.memory[index as usize];

                    for column in 0..width {
                        if (sprite & 0xF) > 0 {
                            let x_pos = self.v[x as usize] + column;
                            let y_pos = self.v[y as usize] + row;
                            if self.renderer.set_pixel(x_pos as isize, y_pos as isize) {
                                self.v[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
                self.pc += 2;
            }

            0xE000 => match opcode & 0xFF {
                0x9E => {
                    if self.renderer.keyboard.is_key_pressed(self.v[x as usize]) > 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }

                0xA1 => {
                    if !self.renderer.keyboard.is_key_pressed(self.v[x as usize]) == 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => {
                    panic!("Instruction not know: {}", opcode);
                }
            },

            0xF000 => match opcode & 0xFF {
                0x07 => {
                    self.v[x as usize] = self.delay_timer;

                    self.pc += 2;
                }

                0x0A => {
                    self.paused = true;

                    while self.renderer.keyboard.keys_pressed.is_empty() {
                        let ten_millis = time::Duration::from_millis(100);
                        thread::sleep(ten_millis);
                    }

                    self.v[x as usize] = self.renderer.keyboard.last_key_pressed;

                    self.pc += 2;
                }

                0x15 => {
                    self.delay_timer = self.v[x as usize];
                    self.pc += 2;
                }

                0x18 => {
                    //ignored
                    //self.soundTimer = self.v[x];
                    self.pc += 2;
                }

                0x1E => {
                    self.i += self.v[x as usize] as u16;
                    self.pc += 2;
                }

                0x29 => {
                    self.i = (self.v[x as usize] * 5) as u16;
                    self.pc += 2;
                }

                0x33 => {
                    // Get the hundreds digit and place it in I.
                    let hundred = self.v[x as usize] / 100;

                    self.memory[self.i as usize] = hundred;

                    // Get tens digit and place it in I+1. Gets a value between 0 and 99,
                    // then divides by 10 to give us a value between 0 and 9.
                    let memory_1 = self.i + 1;

                    let dezen = (self.v[x as usize] % 100) / 10;
                    self.memory[memory_1 as usize] = dezen;

                    // Get the value of the ones (last) digit and place it in I+2.
                    let memory_2 = self.i + 2;
                    let unit = self.v[x as usize] % 10;
                    self.memory[memory_2 as usize] = unit;
                    self.pc += 2;
                }

                0x55 => {
                    for register_index in 0..x {
                        let memory_index = self.i + register_index;

                        self.memory[memory_index as usize] = self.v[register_index as usize];
                    }
                    self.pc += 2;
                }

                0x65 => {
                    for register_index in 0..x {
                        let v_index = self.i + register_index;

                        self.v[register_index as usize] = self.memory[v_index as usize];
                    }
                    self.pc += 2;
                }

                _ => {
                    panic!("Instruction not know: {}", opcode);
                }
            },
            _ => {
                panic!("Instruction not know: {}", opcode);
            }
        }
    }
}
