mod renderer;
mod keyboard;
mod chip8;

use renderer::Renderer;


pub fn main() -> Result<(), String> {
    let mut renderer = Renderer::new(12);
    
    return renderer.show();
}