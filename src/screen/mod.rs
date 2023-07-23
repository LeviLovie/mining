extern crate minifb;
use minifb::{Window, WindowOptions, Scale};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub mod vram;

pub struct Screen {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
    pub Scale: i32,
    pub Title: String,
    pub CloseRequested: bool,
    pub Window: Window,
    pub VRAM: Arc<Mutex<vram::VRAM>>,
    pub Debug: bool,
    pub Debug_Time: bool,
    pub FPS: i32,
    pub FPS_auto_adjust: bool,
    pub Min_sleep_time: u128,
    pub FPS_max: i32,
}
impl Screen {
    pub fn new(width: i32, height: i32, scale: i32, fps: i32, title: &str, debug: bool, debug_time: bool, fps_auto_adjust: bool, min_sleep_time: u128, fps_max: i32) -> Screen {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X1,
        };
        let window = Window::new(&title, width as usize, height as usize, window_options).unwrap();
        return Screen {
            X: 0,
            Y: 0,
            Width: width,
            Height: height,
            Scale: scale,
            Title: title.to_string(),
            CloseRequested: false,
            Window: window,
            VRAM: Arc::new(Mutex::new(vram::VRAM::new(width, height))),
            Debug: debug,
            Debug_Time: debug_time,
            FPS: fps,
            FPS_auto_adjust: fps_auto_adjust,
            Min_sleep_time: min_sleep_time,
            FPS_max: fps_max,
        }
    }

    pub fn run(&mut self) {
        let vram_mut = Arc::clone(&self.VRAM);
        let mut max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64);
        let mut sleep_time: Duration = Duration::from_micros(2000);
        let mut start_time: Instant;
        let mut work_time: Duration;

        while self.Window.is_open() {
            start_time = Instant::now();
            if self.FPS_auto_adjust {
                if sleep_time.as_micros() > self.Min_sleep_time { if self.FPS < self.FPS_max { self.FPS += 1; max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64); } } else if sleep_time.as_micros() < self.Min_sleep_time { if self.FPS != 1 { self.FPS -= 1; max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64); } }
            }
            let vram = vram_mut.lock().unwrap();

            // UPDATE

            self.Window.update_with_buffer(&vram.buffer);
            if self.CloseRequested { break }
            work_time = start_time.elapsed();
            self.debug(max_update_time, work_time, sleep_time, self.Debug, self.Debug_Time);
            if work_time <= max_update_time { sleep_time = max_update_time - work_time } else {sleep_time = Duration::from_micros(0) }
            if sleep_time.as_micros() > 0 { std::thread::sleep(sleep_time) }
            drop(vram);
        }
    }
    fn debug(&mut self, max_update_time: Duration, work_time: Duration, sleep_time: Duration, debug: bool, debug_time: bool) {
        if debug {
            let fps: String; if work_time.as_micros() > 0 { fps = format!("{}", 1_000_000 / max_update_time.as_micros()) } else { fps = "Approx. Infinity".to_string() }
            self.Window.set_title(format!("{}; {} FPS", self.Title, fps).as_str());
            if debug_time {
                let mut fre = 0; if max_update_time.as_micros() > work_time.as_micros() { fre = max_update_time.as_micros() - work_time.as_micros() }
                println!("MAX: {:>5}us; REAL: {:>5}us; FREE: {:>5}us, SLEEP: {:>5}us", max_update_time.as_micros(), work_time.as_micros(), fre, sleep_time.as_micros());
            }
        }
    }
}
