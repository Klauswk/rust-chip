use sdl2::keyboard::Keycode;

pub struct Keyboard {

}

impl Keyboard {

    pub fn new() -> Keyboard {
        return Keyboard{};
    }

    pub fn is_key_pressed(&self, _keycode: u8) -> u8 {
        return 0;
    }


}

pub fn get_pressed(keycode: Keycode) -> u8 {
    match keycode {
        Keycode::Num1 =>  0x1,
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
        _ => 0x10
    }
}
