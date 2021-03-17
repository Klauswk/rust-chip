use sdl2::keyboard::Keycode;

pub struct Keyboard {
    pub keys_pressed: Vec<u8>,
    pub last_key_pressed: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        return Keyboard {
            keys_pressed: Vec::new(),
            last_key_pressed: 0,
        };
    }

    pub fn is_key_pressed(&mut self, keycode: u8) -> u8 {
        let is_pressed = self.keys_pressed.iter().find(|&key| key == &keycode);

        return *is_pressed.unwrap_or(&0);
    }

    pub fn on_key_down(&mut self, keycode: Keycode) {
        let key = self.get_pressed(keycode);
        self.keys_pressed.push(key);
        self.last_key_pressed = key;
    }

    pub fn on_key_up(&mut self, keycode: Keycode) {
        let key = self.get_pressed(keycode);
        if let Some(pos) = self.keys_pressed.iter().position(|x| *x == key) {
            self.keys_pressed.remove(pos);
        }
        self.last_key_pressed = 0;
    }

    pub fn get_pressed(&self, keycode: Keycode) -> u8 {
        match keycode {
            Keycode::Num1 => 0x1,
            Keycode::Num2 => 0x2,
            Keycode::Num3 => 0x3,
            Keycode::Num4 => 0xc,
            Keycode::Q => 0x4,
            Keycode::W => 0x5,
            Keycode::E => 0x6,
            Keycode::R => 0xD,
            Keycode::A => 0x7,
            Keycode::S => 0x8,
            Keycode::D => 0x9,
            Keycode::F => 0xE,
            Keycode::Z => 0xA,
            Keycode::X => 0x0,
            Keycode::C => 0xB,
            Keycode::V => 0xF,
            Keycode::Escape => 0xFF,
            _ => 0x10,
        }
    }
    
}