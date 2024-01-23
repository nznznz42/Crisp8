use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::consts::{SCALE_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::memory::Memory;
use crate::vm::Vm;

pub struct Display {
    screen: Canvas<Window>
}

impl Display {
    pub fn new(context: &sdl2::Sdl) -> Self {
        let video_subsys = context.video().unwrap();
        let window = video_subsys
            .window(
                "Crisp8",
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        return Self {
            screen: canvas
        }
    }

    pub fn draw(&mut self, mem: &Memory, vm: &Vm) {
        let cell_width = self.screen.window().drawable_size().0 / SCREEN_WIDTH as u32;
        let cell_height = self.screen.window().drawable_size().1 / SCREEN_HEIGHT as u32;

        self.screen.set_draw_color(Color::RGB(0, 0, 0));
        self.screen.clear();

        self.screen.set_draw_color(Color::RGB(255, 255, 255));

        for (idx, pixel) in mem.vram.iter().enumerate() {
            if *pixel {
                let x = (idx % SCREEN_WIDTH) * cell_width as usize;
                let y = (idx / SCREEN_WIDTH) * cell_height as usize;

                let rect = Rect::new((x * SCALE_FACTOR) as i32, (y * SCALE_FACTOR) as i32, cell_width, cell_height);
                self.screen.fill_rect(rect).unwrap();
            }
        }
        self.screen.present();
    }
}