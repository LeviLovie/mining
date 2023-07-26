// use std::sync::Mutex;
// use std::{sync::Arc, thread};
// use std::time::{Duration, Instant};

// fn write_text(vram: &mut crate::screen::vram::VRAM, x: i32, y: i32, y_add: i32, gap: i32, letter_size: i32, color: u32, font: &Vec<u32>, text: &str) -> i32 {
//     vram.write_text(x as usize, (y + y_add + gap) as usize, color, font.clone(), text);
//     return y_add + gap + letter_size;
// }

// pub fn Run(vram_mut: &Arc<Mutex<crate::screen::vram::VRAM>>, input_mut: &Arc<Mutex<crate::screen::input::Input>>) -> u32 {
//     let max_update_time = Duration::from_micros(33_333);
//     let font_image = crate::screen::vram::get_font_image();
//     let mut start_time: Instant;
//     let mut work_time: Duration;
//     let mut sleep_time: Duration;

//     let mut vram = vram_mut.lock().unwrap(); vram.clear();
//     let mut static_pointer_y: i32 = 0;
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xAAAAAA, &font_image, "To return press the 'TAB' key.");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xFFFFFF, &font_image, "Mining game license for current version of software:");

//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 20, 8, 0xFFFFFFF, &font_image, "MIT License");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "Copyright (c) 2023 leviiloviee");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "Permission is hereby granted, free of charge, to any person obtaining a copy");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "of this software and associated documentation files (the \"Software\"), to deal");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "in the Software without restriction, including without limitation the rights");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "to use, copy, modify, merge, publish, distribute, sublicense, and/or sell");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "copies of the Software, and to permit persons to whom the Software is");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "furnished to do so, subject to the following conditions:");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "The above copyright notice and this permission notice shall be included in all");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "copies or substantial portions of the Software.");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,");
//     static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE");
//     _ = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFFFFFFF, &font_image, "SOFTWARE.");
//     drop(vram);

//     loop {
//         start_time = Instant::now();
//         let input = input_mut.lock().unwrap(); if input.Tab { return 1; } drop(input);
//         work_time = start_time.elapsed();     
//         if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
//         else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
//     }
// }