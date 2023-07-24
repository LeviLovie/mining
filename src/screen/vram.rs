use image::Rgb;

const FONT_LETTERS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz(|)~";
const LETTER_SIZE: usize = 8;
const FONT_IMAGE_WIDTH: i32 = 160;
const FONT_IMAGE_HEIGTH: i32 = 40;

pub struct VRAM {
    pub size_x: i32,
    pub size_y: i32,
    pub buffer: Vec<u32>,
    pub modified: bool,
}
impl VRAM {
    pub fn new(size_x: i32, size_y: i32) -> VRAM {
        VRAM {
            size_x,
            size_y,
            buffer: vec![0x000000; (size_x * size_y) as usize],
            modified: true,
        }
    }
    pub fn calc_index(&self, x: i32, y: i32) -> i32 {
        (y * self.size_x + x) as i32
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= self.size_x || y >= self.size_y {return;}
        let index = self.calc_index(x, y);
        self.buffer[index as usize] = color;
        self.modified = true;
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
        if x >= self.size_x || y >= self.size_y {return 0x000000;}
        let index = self.calc_index(x, y);
        self.buffer[index as usize]
    }
    pub fn fill(&mut self, color: u32) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = color;
        }
        self.modified = true;
    }
    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0x000000;
        }
        self.modified = true;
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for i in x..x+w {
            for j in y..y+h {
                self.set_pixel(i, j, color);
            }
        }
    }
    pub fn circle(&mut self, x: i32, y: i32, r: i32, color: u32) {
        for i in x-r..x+r {
            for j in y-r..y+r {
                if (i-x)*(i-x) + (j-y)*(j-y) <= r*r {
                    self.set_pixel(i, j, color);
                }
            }
        }
    }
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 {1} else {-1};
        let sy = if y1 < y2 {1} else {-1};
        let mut err = dx - dy;
        loop {
            self.set_pixel(x as i32, y as i32, color);
            if x == x2 as i32 && y == y2 as i32 {break;}
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn write_text(&mut self, x: usize, y: usize, color: u32, text: &str) {
        let text_length = text.len();

        let image_data = crate::include::FONT_PNG;
        let image: image::DynamicImage = image::load_from_memory(image_data).unwrap();
        let img = image.to_rgb32f();

        for i in 0..text_length {
            for j in 0..FONT_LETTERS.len() {
                if text.chars().nth(i) == FONT_LETTERS.chars().nth(j) {
                    let letter_x = (j * LETTER_SIZE) % 160;
                    let mut letter_y = 0;
                    for _ in 0..(j * LETTER_SIZE) / 160 {
                        letter_y += LETTER_SIZE;
                    }

                    for k in 0..LETTER_SIZE {
                        for l in 0..LETTER_SIZE {
                            let img_pixel = img.get_pixel(letter_x as u32 + l as u32, letter_y as u32 + k as u32);
                            if img_pixel == &Rgb([0.0, 0.0, 0.0]) {
                                self.set_pixel((x + l + (i * LETTER_SIZE)) as i32, (y + k) as i32, color);
                            }
                        }
                    }
                }
            }
        }
    }
}
