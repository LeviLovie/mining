use std::{sync::Arc, sync::Mutex, thread};
use std::time::{Duration, Instant};

fn write_text(vram: &mut crate::screen::vram::VRAM, x: i32, y: i32, y_add: i32, gap: i32, letter_size: i32, color: u32, text: &str) -> i32 {
    vram.write_text(x as usize, (y + y_add + gap) as usize, color, text);
    return y_add + gap + letter_size;
}

pub fn Run(vram_mut: &Arc<Mutex<crate::screen::vram::VRAM>>, input_mut: &Arc<Mutex<crate::screen::input::Input>>) -> u32 {
    let max_update_time = Duration::from_micros(33_333);
    let mut start_time: Instant;
    let mut work_time: Duration;
    let mut sleep_time: Duration;

    let mut vram = vram_mut.lock().unwrap(); vram.clear();
    let mut static_pointer_y: i32 = 0;
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xFF0000, "NOT IMPLEMENTED YET.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xFFFFFF, "But it is the menu of the game.");
    _ = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0x00FF00, "Press the 'ECS' key to exit.");
    drop(vram);

    loop {
        start_time = Instant::now(); let vram = vram_mut.lock().unwrap();
        let input = input_mut.lock().unwrap(); if input.Esc { std::process::exit(0); } drop(input);
    
        let input = input_mut.lock().unwrap().zeros(); drop(input);
        drop(vram); work_time = start_time.elapsed();     
        if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
        else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
    }
}