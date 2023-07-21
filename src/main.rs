#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod screen;

use std::env;

fn main() {
    // Parse debug arguments into the debug variable
    let args: Vec<String> = env::args().collect();
    let mut debug = false; for arg in &args { if arg == "-d" { debug = true } }
    let mut debug_time = false; for arg in &args { if arg == "-t" { debug_time = true } }
    if debug { println!("{} v{}, in debug mode", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")) }

    // Create window
    let mut window = screen::Screen::new(240, 160, 3, 30, "Hello, world!", debug, debug_time);
    window.run();
}
