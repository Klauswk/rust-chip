mod chip8;
mod cpu;
mod keyboard;
mod renderer;

use chip8::Chip8;
use cpu::Cpu;
use keyboard::Keyboard;
use renderer::Renderer;

pub fn main() -> Result<(), String> {
    let keyboard = Keyboard::new();
    let renderer = Renderer::new(12, keyboard)?;

    let cpu = Cpu::new(renderer);
    let mut chip8 = Chip8::new(cpu);
    return chip8.run();
}
