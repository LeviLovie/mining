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
}
