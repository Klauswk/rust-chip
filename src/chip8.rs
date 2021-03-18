use crate::cpu::Cpu;
use std::{thread, time};
pub struct Chip8 {
    pub cpu: Cpu,
}

impl Chip8 {
    pub fn new(mut cpu: Cpu) -> Chip8 {
        cpu.load_sprites();
        cpu.load_program();

        return Chip8 { cpu };
    }

    pub fn run(&mut self) -> Result<(), String> {
        let ten_millis = time::Duration::from_millis(100);

        'running: loop {
            if self.cpu.delay_timer > 0 {
                self.cpu.delay_timer -= 1;
            }

            let result = self.cpu.cycle();
            
            if result > 0 {
                break 'running;
            }

            thread::sleep(ten_millis);
        }

        Ok(())
    }
}
