use crate::renderer::Renderer;
use std::fs::File;
use std::io::prelude::*;

pub struct Cpu {
    pub renderer: Renderer,
    pub memory: Vec<u8>,
    pub delay_timer: u8,
    pub v: Vec<u8>,
    pub i: u8,
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
            self.memory.insert(i, x.clone());
            i = i + 1;
        }
    }

    pub fn load_program(&mut self) {
        let mut file = File::open("roms/BLITZ").unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let mut i = 0;

        for dat in data {
            self.memory.insert(512 + i, dat);
            i = i + 1;
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

    fn execute_instruction(&mut self, opcode: u16) {
        self.pc = self.pc + 2;

        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.renderer.clear(),
                0x00EE => {
                    self.pc = self.stack.pop().unwrap();
                }
                _ => {
                    println!("Instruction not know: {}", opcode);
                    //panic!("Instruction not know: {}", opcode);
                }
            },
            0x1000 => {
                self.pc = opcode & 0xFFF;
            }

            0x2000 => {
                self.stack.push(self.pc);
                self.pc = opcode & 0xFFF;
            }

            0x3000 => {
                if self.v[x as usize] == (opcode as u8 & 0xFF) {
                    self.pc += 2;
                }
            }

            0x4000 => {
                if self.v[x as usize] != (opcode as u8 & 0xFF) {
                    self.pc += 2;
                }
            }

            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }

            0x6000 => {
                self.v[x as usize] = opcode as u8 & 0xFF;
            }

            0x7000 => {
                self.v[x as usize] = self.v[x as usize] + opcode as u8 & 0xFF;
            }

            0x8000 => match opcode & 0xF {
                0x0 => {
                    self.v[x as usize] = self.v[y as usize];
                }

                0x1 => {
                    self.v[x as usize] |= self.v[y as usize];
                }

                0x2 => {
                    self.v[x as usize] &= self.v[y as usize];
                }

                0x3 => {
                    self.v[x as usize] ^= self.v[y as usize];
                }

                0x4 => {
                    let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;

                    if sum > 0xff {
                        self.v[0xf] = 1;
                    } else {
                        self.v[0xf] = 0;
                    }

                    self.v[x as usize] = sum as u8;
                }

                0x5 => {
                    self.v[0xF] = 0;

                    if self.v[x as usize] > self.v[y as usize] {
                        self.v[0xF] = 1;
                    }

                    self.v[x as usize] = self.v[x as usize] - self.v[y as usize];
                }

                0x6 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;

                    self.v[x as usize] >>= 1;
                }

                0x7 => {
                    self.v[0xF] = 0;

                    if self.v[y as usize] > self.v[x as usize] {
                        self.v[0xF] = 1;
                    }

                    self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                }

                0xE => {
                    self.v[0xF] = self.v[x as usize] & 0x80;
                    self.v[x as usize] <<= 1;
                }
                _ => {
                    panic!("Instruction not know: {}", opcode);
                }
            },

            0x9000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }

            0xA000 => {
                self.i = opcode as u8 & 0xFF;
            }

            0xB000 => {
                self.pc = (opcode & 0xFF) + self.v[0] as u16;
            }

            0xC000 => {
                let ran: u8 = rand::random();

                self.v[x as usize] = ran & (opcode as u8 & 0xff);
            }

            0xD000 => {
                let width: u8 = 8;
                let height: u8 = opcode as u8 & 0xf;

                self.v[0xF] = 0;

                for row in 0..height {
                    let index = self.i + row;
                    let mut sprite = self.memory[index as usize];

                    for column in 0..width {
                        if (sprite & 0x80) > 0 {
                            let x_pos = self.v[x as usize] + column;
                            let y_pos = self.v[y as usize] + row;
                            if self.renderer.set_pixel(x_pos as isize, y_pos as isize) {
                                self.v[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
            }

            0xE000 => match opcode & 0xFF {
                0x9E => {
                    //REVISAR
                    if self.renderer.keyboard.is_key_pressed(self.v[x as usize]) > 0 {
                        self.pc += 2;
                    }
                }

                0xA1 => {
                    if self.renderer.keyboard.is_key_pressed(self.v[x as usize]) == 0 {
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
                }

                0x0A => {
                    self.paused = true;
                }

                0x15 => {
                    self.delay_timer = self.v[x as usize];
                }

                0x18 => {
                    //ignored
                    //self.soundTimer = self.v[x];
                }

                0x1E => {
                    self.i += self.v[x as usize];
                }

                0x29 => {
                    self.i = self.v[x as usize] * 5;
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
                }

                0x55 => {
                    for register_index in 0..x {
                        let memory_index = self.i + register_index as u8;

                        self.memory[memory_index as usize] = self.v[register_index as usize];
                    }
                }

                0x65 => {
                    for register_index in 0..x {
                        let v_index = self.i + register_index as u8;

                        self.v[register_index as usize] = self.memory[v_index as usize];
                    }
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
