use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::convert::TryInto;
use crate::keyboard::get_pressed;


const ROWS: isize = 32;
const COLUMNS: isize = 64; 

#[derive(Debug)]
pub struct Renderer {
    scale: isize,
    display: Vec<u32>,
}

impl Renderer {

    pub fn new(scale: isize) -> Renderer {
        let mut display: Vec<u32> = Vec::new();

        for n in 0..ROWS*COLUMNS {
            display.push(0);
        }

        return Renderer {
            scale,
            display
        }
    }

    pub fn show(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        
        for n in 0..ROWS*COLUMNS {
            self.display.push(0);
        }
    
        let window = video_subsystem
            .window("Rust Chip", (COLUMNS * &self.scale).try_into().unwrap(), ((ROWS * &self.scale)).try_into().unwrap())
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
    
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump()?;
    
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
    
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...

            for i in 0..ROWS*COLUMNS {
                // Grabs the x position of the pixel based off of `i`
                let x = (i % COLUMNS) * self.scale;
        
                // Grabs the y position of the pixel based off of `i`
                let y = (((i / COLUMNS) * self.scale) as f32).floor() as isize;
        
                canvas.set_draw_color(Color::WHITE);
                // If the value at this.display[i] == 1, then draw a pixel.

                let t = i as usize;
                /*if self.display[t] > 0 {
        
                    // Place a pixel at position (x, y) with a width and height of scale
                    canvas.fill_rect(Rect::new(x.try_into().unwrap(), y.try_into().unwrap(), self.scale as u32, self.scale as u32));
                }*/

                canvas.fill_rect(Rect::new(0, 0, self.scale as u32, self.scale as u32));
                canvas.fill_rect(Rect::new(0, 1, self.scale as u32, self.scale as u32));
                canvas.fill_rect(Rect::new(0, 2, self.scale as u32, self.scale as u32));
                canvas.fill_rect(Rect::new(0, 3, self.scale as u32, self.scale as u32));
                canvas.fill_rect(Rect::new(0, 4, self.scale as u32, self.scale as u32));
                canvas.fill_rect(Rect::new(0, 5, self.scale as u32, self.scale as u32));
            }
            canvas.present();
        }
    
        Ok(())
    }

    fn setPixel(&mut self, mut x: isize, mut y: isize) -> bool {
        if x > COLUMNS {
            x -= COLUMNS;
        } else if x < 0 {
            x += COLUMNS;
        }
        
        if y > ROWS {
            y -= ROWS;
        } else if y < 0 {
            y += ROWS;
        }
        
        let xPos = x as usize;
        let yPos = (y * COLUMNS) as usize;
        let pixel_loc: usize = xPos + yPos;

        self.display[pixel_loc] ^= 1;

        return self.display[pixel_loc] > 0;
    }

    fn clear(&mut self) {
        self.display.clear();
        for n in 0..ROWS*COLUMNS {
            self.display.push(0);
        }
    }
}
