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
    pub FPS: i32,
    pub Window: Window,
    pub VRAM: Arc<Mutex<vram::VRAM>>,
    pub Debug: bool,
    pub Debug_Time: bool,
}
impl Screen {
    pub fn new(width: i32, height: i32, scale: i32, fps: i32, title: &str, debug: bool, debug_time: bool) -> Screen {
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
            FPS: fps,
            Window: window,
            VRAM: Arc::new(Mutex::new(vram::VRAM::new(width, height))),
            Debug: debug,
            Debug_Time: debug_time,
        }
    }

    pub fn run(&mut self) {
        let vram_mut = Arc::clone(&self.VRAM);
        let max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64);
        let mut sleep_time: Duration;
        let mut start_time: Instant;
        let mut work_time: Duration;

        while self.Window.is_open() {
            start_time = Instant::now();
            let vram = vram_mut.lock().unwrap();

            self.Window.update_with_buffer(&vram.buffer);
            if self.CloseRequested { break }
            work_time = start_time.elapsed();
            if self.Debug_Time { self.debug(max_update_time, work_time) }
            if work_time <= max_update_time { sleep_time = max_update_time - work_time } else { work_time = max_update_time; sleep_time = Duration::from_micros(0) }
            if sleep_time.as_micros() > 0 { std::thread::sleep(sleep_time) }
            drop(vram);
        }
    }
    fn debug(&mut self, max_update_time: Duration, work_time: Duration) {
        let fps: String; if work_time.as_micros() > 0 { fps = format!("{}", 1_000_000 / max_update_time.as_micros()) } else { fps = "Approx. Infinity".to_string() }
        self.Window.set_title(format!("{}; {} FPS", self.Title, fps).as_str());
        let mut fre = 0; if max_update_time.as_micros() > work_time.as_micros() { fre = max_update_time.as_micros() - work_time.as_micros() }
        println!("MAX: {:>5}us; REL: {:>5}us; FRE: {:>5}us", max_update_time.as_micros(), work_time.as_micros(), fre);
    }
}
