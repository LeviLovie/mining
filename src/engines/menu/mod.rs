// use std::{sync::Arc, sync::Mutex, thread};
// use std::time::{Duration, Instant};

// fn write_text(vram: &mut crate::screen::vram::VRAM, x: i32, y: i32, y_add: i32, gap: i32, letter_size: i32, color: u32, font: Vec<u32>, text: &str) -> i32 {
//     vram.write_text(x as usize, (y + y_add + gap) as usize, color, font, text);
//     return y_add + gap + letter_size;
// }
// fn write_text_on_center(vram: &mut crate::screen::vram::VRAM, y: i32, y_add: i32, gap: i32, letter_size: i32, color: u32, screen_size: i32, font: &Vec<u32>, text: &str) -> i32 {
//     write_text(vram, left_amout_to_make_str_on_center(screen_size, letter_size, text), y, y_add, gap, letter_size, color, font.clone(), text);
//     return y_add + gap + letter_size;
// }
// fn left_amout_to_make_str_on_center(screen_size_x: i32, letter_size: i32, text: &str) -> i32 {
//     return (screen_size_x - (letter_size * text.len() as i32)) / 2;
// }

// pub fn Run(vram_mut: &Arc<Mutex<crate::screen::vram::VRAM>>, input_mut: &Arc<Mutex<crate::screen::input::Input>>, size_x: i32) -> u32 {
//     let max_update_time = Duration::from_micros(66_666);
//     let mut start_time: Instant;
//     let mut work_time: Duration;
//     let mut sleep_time: Duration;

//     let mut vram = vram_mut.lock().unwrap(); vram.clear();
//     drop(vram);

//     loop {
//         start_time = Instant::now(); let mut vram = vram_mut.lock().unwrap();
//         let input = input_mut.lock().unwrap(); if input.Esc { std::process::exit(0); } drop(input);
//         let mut static_pointer_y: i32 = 0; vram.fill(0x000000);
//         let font_image = crate::screen::vram::get_font_image();
        
//         // vram.rect(left_amout_to_make_str_on_center(size_x, 8, "|\\/|  |  |\\ |  |  |\\ | / _   / _  /\\  |\\/| |_"), 10, (size_x - "|\\/|  |  |\\ |  |  |\\ | / _   / _  /\\  |\\/| |_".len() as i32) / 2, 27, 0xFFFFFF);
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 10, 8, 0x00FF00, size_x, &font_image, "    ___      ___       __");
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 1, 8, 0x00FF00, size_x, &font_image, "|\\/|  |  |\\ |  |  |\\ | / _ ");
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 1, 8, 0x00FF00, size_x, &font_image, "|  | _|_ | \\| _|_ | \\| \\_| "); 

//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 40, 8, 0x00FF00, size_x, &font_image, "Play"); 
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 1, 8, 0x00FF00, size_x, &font_image, "Options"); 
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 1, 8, 0x00FF00, size_x, &font_image, "Recover ('R' key)"); 
//         static_pointer_y = write_text_on_center(&mut vram, 0, static_pointer_y, 1, 8, 0x00FF00, size_x, &font_image, "Exit");
        
//         let input = input_mut.lock().unwrap().zeros(); drop(input);
//         drop(vram); work_time = start_time.elapsed();     
//         if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
//         else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
//     }
// }