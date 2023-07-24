use std::{sync::Arc, sync::Mutex, thread};
use std::time::{Duration, Instant};

fn write_text(vram: &mut crate::screen::vram::VRAM, x: i32, y: i32, y_add: i32, gap: i32, letter_size: i32, color: u32, text: &str) -> i32 {
    vram.write_text(x as usize, (y + y_add + gap) as usize, color, text);
    return y_add + gap + letter_size;
}

pub fn Run(vram_mut: &Arc<Mutex<crate::screen::vram::VRAM>>, input_mut: &Arc<Mutex<crate::screen::input::Input>>) -> u32 {
    let max_update_time = Duration::from_micros(33_333);
    let mut iteration: u128 = 0;
    let mut start_time: Instant;
    let mut work_time: Duration;
    let mut sleep_time: Duration;

    let mut vram = vram_mut.lock().unwrap(); vram.clear();
    let mut static_pointer_y: i32 = 0;
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 0, 8, 0x3333FF, "      ___      ___       __    __            _ ");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 0, 8, 0x3333FF, " |\\/|  |  |\\ |  |  |\\ | / _   / _  /\\  |\\/| |_ ");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 0, 8, 0x3333FF, " |  | _|_ | \\| _|_ | \\| \\_|   \\_| /--\\ |  | |_ ");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 0, 8, 0x3333FF, "                                               ");

    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xCCCCCC, "Game fully coded in Rust winthout any engine with MiniFB GUI library.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xCCCCCC, "MIT License Copyright (c) 2023 leviiloviee.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xCCCCCC, "Code, art, and desing on the GitHub:");
    static_pointer_y = write_text(&mut vram, 20, 0, static_pointer_y, 1, 8, 0xCCCCCC, "https://github.com/LeviiLovie/mining");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xCCCCCC, "Latest versions can be also downloaded from Github:");
    static_pointer_y = write_text(&mut vram, 20, 0, static_pointer_y, 1, 8, 0xCCCCCC, "https://github.com/LeviiLovie/mining/releases");

    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 21, 8, 0xFF0000, "BY PRESSING SPACE AND CONTINUING YOU AGREE,");
    // static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFF0000, "WHAT ACCEPTS THE LICENSE AND THE USER AGREEMENT.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFF0000, "WHAT ACCEPTS THE LICENSE.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xFF3333, "IF YOU NOT AGREE, CLOSE THE APPLICATION.");
    static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 3, 8, 0xFF3333, "To see license press the 'L' key.");
    // static_pointer_y = write_text(&mut vram, 0, 0, static_pointer_y, 1, 8, 0xFF3333, "To see user agreement press the 'A' key.");
    drop(vram);

    loop {
        start_time = Instant::now(); let mut vram = vram_mut.lock().unwrap();

        if iteration == 25 {
            _ = write_text(&mut vram, 0, 0, static_pointer_y, 21, 8, 0xFFFFFF, "To continue press the 'SPACE' key.");
        }
        let input = input_mut.lock().unwrap();
        if iteration > 25 { if input.Space { return 2; } }
        if input.Esc { std::process::exit(0); }
        if input.L { return 3; }
        // if input.A { return 4; }
        drop(input);

        let input = input_mut.lock().unwrap().zeros(); drop(input);

        iteration += 1; drop(vram); work_time = start_time.elapsed();     
        if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
        else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
    }
}