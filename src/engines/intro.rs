use std::sync::Mutex;
use std::{sync::Arc, thread};
use std::time::{Duration, Instant};

use crate::screen::color::*;

pub fn Run(vram_mut: Arc<Mutex<crate::screen::vram::VRAM>>, _size_x: i32, _size_y: i32, _data: &str) {
    let max_update_time = Duration::from_micros(100_000);
    let mut _iteration: u128 = 0;
    let mut start_time: Instant;
    let mut work_time: Duration;
    let mut sleep_time: Duration;

    let colors_amount = (COLORS.len() as i32 / 2) - 1;
    let rect_size = 10;
    let palette_size = rect_size * colors_amount;
    let palettes_in_screen_horizontaly = (_size_x as f32 / palette_size as f32).ceil() as i32;
    let palettes_in_screen_vertically = (_size_y as f32 / rect_size as f32).ceil() as i32;
    loop {
        start_time = Instant::now(); let mut vram = vram_mut.lock().unwrap(); vram.clear();

        let iteration_part: i32 = _iteration as i32 / 2;
        
        for y in 0..palettes_in_screen_vertically {
            for x in 0..palettes_in_screen_horizontaly {
                for i in 0..colors_amount {
                    let mut x_pos = (((x * palette_size) + (i as i32 * rect_size)) + (y * rect_size)) + (iteration_part * rect_size);
                    if x_pos >= _size_x { x_pos = x_pos - _size_x; }
                    vram.rect(x_pos, y as i32 * rect_size, rect_size, rect_size, COLORS[i as usize]);
                }
            }
        }

        _iteration += 1; drop(vram); work_time = start_time.elapsed();     
        if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
        else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
    }
}