use std::{sync::Arc, sync::Mutex, thread};
use std::time::{Duration, Instant};

pub fn Run(
    vram_mut: &Arc<Mutex<crate::screen::vram::VRAM>>,
    input_mut: &Arc<Mutex<crate::screen::input::Input>>
) -> u32 {
    let ( mut max_ut, mut iter, mut start_t, mut work_t, mut sleep_t
    ) = ( Duration::from_micros(33_333), 0, Instant::now(), Duration::from_micros(33_333), Duration::from_micros(33_333));

    let mut vram = vram_mut.lock().unwrap(); vram.clear();
    // BEFORE CYCLE UPDATE
    drop(vram);

    loop {
        start_time = Instant::now();
        let ( mut vram, mut input
        ) = ( vram_mut.lock().unwrap(), input_mut.lock().unwrap() );

        // IN CYCLE UPDATE

        iter += 1; drop(vram); drop(input);
        if max_update_time.as_micros() >= work_time.as_micros() { sleep_time = max_update_time - work_time; thread::sleep(sleep_time) }
        else { println!("Warning: Update time is too long: {:>5}us (max is {:>5}us)", work_time.as_micros(), max_update_time.as_micros()) }
    }
}