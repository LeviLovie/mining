#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::{sync::Arc, thread};

pub mod screen; use screen::*;
pub mod include; use include::*;
pub mod engines;

fn main() {
    let debug_config = yaml::get_config_from_file("debug", CONFIG_FILE);
    let screen_config = yaml::get_config_from_file("screen", CONFIG_FILE);
    let fps_screen_config = yaml::get_config("fps", &screen_config);
    let mut screen = Screen::new(
        yaml::get_i32(&screen_config, "size_x"),
        yaml::get_i32(&screen_config, "size_y"),
        yaml::get_i32(&fps_screen_config, "target_fps"),
        yaml::get_string(&screen_config, "title").as_str(),
        yaml::get_bool(&debug_config, "debug_global"),
        yaml::get_bool(&debug_config, "debug_time_info"),
        yaml::get_bool(&fps_screen_config, "fps_auto_adjust"),
        yaml::get_u128(&fps_screen_config, "min_update_sleep_time"),
        yaml::get_i32(&fps_screen_config, "max_fps")
    );

    let vram_mut = Arc::clone(&screen.VRAM);
    let input_mut = Arc::clone(&screen.Input);
    let _ = thread::Builder::new().name("engines".to_string()).spawn(move || {
        let mut mode: u32 = 2;
        loop {
            if mode == 1 { mode = engines::intro::Run(&vram_mut, &input_mut); }
            if mode == 2 { mode = engines::menu::Run(&vram_mut, &input_mut, screen.Width) }
            if mode == 3 { mode = engines::license::Run(&vram_mut, &input_mut); }
            // if mode == 3 { mode = engines::agree::Run(&vram_mut, &input_mut); }
        }
    });
    
    screen.run();
}
