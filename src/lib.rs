use logger;
use minifb::{Window, WindowOptions};

pub fn say_hi() {
    logger::log(
        logger::PREFIX_DEBUG,
        format!("Booting {}GFX v{}{} up...",
            logger::COLOR_BOLD_GREEN,
            env!("CARGO_PKG_VERSION"),
            logger::COLOR_RESET,
        ).as_str()
    );
}

pub struct Screen {
    pub size: (usize, usize),
    pub scale: usize,
    pub title: &'static str,
    pub raw_title: &'static str,
    window: Window,
    pub buffer: Vec<u32>,
    pub max_update_time_as_micros: u128,
}
impl Screen {
    pub fn new(height: usize, width: usize, scale: usize, title: &'static str, fps: usize) -> Screen {
        return Screen {
            size: (height, width),
            scale: scale,
            title: title,
            raw_title: title,
            window: Window::new(
                title,
                width * scale,
                height * scale,
                WindowOptions::default(),
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            }),
            buffer: vec![0xFF_000000; width * height],
            max_update_time_as_micros: 1000000 / fps as u128,
        }
    }
    pub fn is_open(&self) -> bool {
        return self.window.is_open();
    }
    pub fn redraw(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.size.1, self.size.0).unwrap();
    }
    pub fn add_to_title(&mut self, text: String) {
        self.window.set_title(format!("{} - {}", self.raw_title, text).as_str());
    }

    pub fn draw_rectangle(&mut self, pos_y: usize, pos_x: usize, size_y: usize, size_x: usize, color: u32) {
        let buffer_width = self.size.1;
        for y in 0..size_y {
            let buffer_row_start = (y + pos_y) * buffer_width;

            for x in 0..size_x {
                if y >= self.size.0 || x >= self.size.1 {
                    continue;
                }
                if x + pos_x >= self.size.1 || y + pos_y >= self.size.0 {
                    break;
                }

                let buffer_index = buffer_row_start + (x + pos_x);

                self.buffer[buffer_index] = color;
            }
        };
    }
    pub fn draw_sprite(&mut self, sprite: &[u32], size_y: usize, size_x: usize, pos_y: usize, pos_x: usize) {
        let buffer_width = self.size.1;
        for y in 0..size_y {
            let buffer_row_start = (y + pos_y) * buffer_width;
            let sprite_row_start = y * size_x;

            for x in 0..size_x {
                if y >= self.size.0 || x >= self.size.1 {
                    continue;
                }
                if x + pos_x >= self.size.1 || y + pos_y >= self.size.0 {
                    break;
                }

                let buffer_index = buffer_row_start + (x + pos_x);
                let sprite_index = sprite_row_start + x;

                self.buffer[buffer_index] = sprite[sprite_index];
            }
        };
    }
}
