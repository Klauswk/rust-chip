mod renderer;
mod keyboard;
mod chip8;
mod cpu;

use renderer::Renderer;
use chip8::Chip8;
use keyboard::Keyboard;
use cpu::Cpu;

pub fn main() -> Result<(), String> {
    let renderer = Renderer::new(12)?;
    let keyboard = Keyboard::new();

    let cpu = Cpu::new(renderer, keyboard);
    let mut chip8 = Chip8::new(cpu);
    return chip8.run();
}