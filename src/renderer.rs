use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::convert::TryInto;
use crate::keyboard::Keyboard;

const ROWS: isize = 32;
const COLUMNS: isize = 64; 

pub struct Renderer {
    pub keyboard: Keyboard,
    scale: isize,
    display: Vec<u32>,
    sdl_context: Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Renderer {

    pub fn new(scale: isize, keyboard: Keyboard) -> Result<Renderer, String> {
        let display: Vec<u32> = vec![0; (ROWS*COLUMNS) as usize];

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Rust Chip", (COLUMNS * scale).try_into().unwrap(), ((ROWS * scale)).try_into().unwrap())
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
    
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        return Ok(Renderer {
            keyboard,
            scale,
            display,
            sdl_context,
            canvas,
        });
    }

    pub fn render(&mut self) -> u8 {

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();
        let mut event_pump = self.sdl_context.event_pump().unwrap();
    
        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {
                    ..
                } => {
                    return 1;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return 1;
                },
                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    self.keyboard.on_key_down(keycode.unwrap());
                },
                Event::KeyUp {
                    keycode,
                    ..
                } => {
                    self.keyboard.on_key_up(keycode.unwrap());
                },
                _ => {
                    return 0;
                }
            }

            /*match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return 1;
                },
                Event::KeyDown {
                    ke
                }
            }*/
        }


        for i in 0..ROWS*COLUMNS {
            // Grabs the x position of the pixel based off of `i`
            let x = (i % COLUMNS) * self.scale;
    
            // Grabs the y position of the pixel based off of `i`
            let y = (((i / COLUMNS) * self.scale) as f32).floor() as isize;
    
            self.canvas.set_draw_color(Color::WHITE);
            // If the value at this.display[i] == 1, then draw a pixel.

            let t = i as usize;
            if self.display[t] > 0 {    
                // Place a pixel at position (x, y) with a width and height of scale
                self.canvas.fill_rect(Rect::new(x.try_into().unwrap(), y.try_into().unwrap(), self.scale as u32, self.scale as u32)).unwrap();
            }

            /*self.canvas.fill_rect(Rect::new(0, 0, self.scale as u32, self.scale as u32));
            self.canvas.fill_rect(Rect::new(0, 1, self.scale as u32, self.scale as u32));
            self.canvas.fill_rect(Rect::new(0, 2, self.scale as u32, self.scale as u32));
            self.canvas.fill_rect(Rect::new(0, 3, self.scale as u32, self.scale as u32));
            self.canvas.fill_rect(Rect::new(0, 4, self.scale as u32, self.scale as u32));
            self.canvas.fill_rect(Rect::new(0, 5, self.scale as u32, self.scale as u32));*/
        }
        self.canvas.present();

        return 0;
    }

    pub fn set_pixel(&mut self, mut x: isize, mut y: isize) -> bool {
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
        
        let x_pos = x as usize;
        let y_pos = (y * COLUMNS) as usize;
        let pixel_loc: usize = x_pos + y_pos;

        self.display[pixel_loc] ^= 1;

        return self.display[pixel_loc] > 0;
    }

    pub fn clear(&mut self) {
        self.display.clear();
        for _n in 0..ROWS*COLUMNS {
            self.display.push(0);
        }
    }
}
