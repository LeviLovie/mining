use std::sync::Mutex;
use std::{sync::Arc, thread};
use std::time::{Duration, Instant};

pub fn Run(vram_mut: Arc<Mutex<crate::screen::vram::VRAM>>, _data: &str) {
    let max_update_time = Duration::from_micros(33_333);
    let mut _iteration: u128 = 0;
    let mut start_time: Instant;
    let mut work_time: Duration;
    let mut sleep_time: Duration;
    loop {
        start_time = Instant::now(); let mut vram = vram_mut.lock().unwrap(); vram.clear();
        
        vram.rect(10, 10, 25, 25, 0xAAAAAAA);
        
        _iteration += 1; drop(vram); work_time = start_time.elapsed();     
        if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time); }
        else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()); }
    }
}