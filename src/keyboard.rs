use std::process::exit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

pub struct Keyboard {
    pub events: sdl2::EventPump,
    pub keys: [bool; 16]
}

impl Keyboard {
    pub fn new(context: &Sdl) -> Self {
        return Self {
            events: context.event_pump().unwrap(),
            keys: [false; 16],
        }
    }

    pub fn poll(&mut self) {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                exit(1);
            };
        }

        let keys: Vec<Keycode> = self.events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        self.keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };
            if let Some(i) = index {
                self.keys[i] = true;
            }
        }
    }

    pub fn is_key_pressed(&self, key: u16) -> bool {
        return self.keys[key as usize]
    }
}