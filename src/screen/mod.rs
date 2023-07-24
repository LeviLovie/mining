extern crate minifb;
use minifb::{Key, Window, WindowOptions, Scale};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub mod vram;
pub mod color;
pub mod input;

pub struct Screen {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
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
    pub Input: Arc<Mutex<input::Input>>,
}
impl Screen {
    pub fn new(width: i32, height: i32, fps: i32, title: &str, debug: bool, debug_time: bool, fps_auto_adjust: bool, min_sleep_time: u128, fps_max: i32) -> Screen {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        };
        let window = Window::new(&title, width as usize, height as usize, window_options).unwrap();
        return Screen {
            X: 0,
            Y: 0,
            Width: width,
            Height: height,
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
            Input: Arc::new(Mutex::new(input::Input::new())),
        }
    }

    pub fn run(&mut self) {
        let vram_mut = Arc::clone(&self.VRAM);
        let mut max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64);
        let mut sleep_time: Duration = Duration::from_micros(2000);
        let mut start_time: Instant;
        let mut work_time: Duration;

        while !self.CloseRequested {
            start_time = Instant::now();
            if self.FPS_auto_adjust {
                if sleep_time.as_micros() > self.Min_sleep_time { if self.FPS < self.FPS_max { self.FPS += 1; max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64); } } else if sleep_time.as_micros() < self.Min_sleep_time { if self.FPS != 1 { self.FPS -= 1; max_update_time = Duration::from_micros(1_000_000 / self.FPS as u64); } }
            }
            let mut vram = vram_mut.lock().unwrap();

            self.keypresses(Arc::clone(&self.Input));

            self.Window.update_with_buffer(&vram.buffer, self.Width as usize, self.Height as usize).unwrap(); vram.modified = false;
            drop(vram);
            if self.CloseRequested { break }
            work_time = start_time.elapsed();
            let title : String = debug(max_update_time, work_time, sleep_time, self.Debug, self.Debug_Time, self.Title.to_string());
            self.Window.set_title(&title);
            if work_time <= max_update_time { sleep_time = max_update_time - work_time } else {sleep_time = Duration::from_micros(0) }
            if sleep_time.as_micros() > 0 { std::thread::sleep(sleep_time) }
        }
    }

    fn keypresses(&mut self, input: Arc<Mutex<input::Input>>) {
        if self.Window.is_key_down(Key::A) { input.lock().unwrap().A = true; } else { input.lock().unwrap().A = false; }
        if self.Window.is_key_down(Key::B) { input.lock().unwrap().B = true; } else { input.lock().unwrap().B = false; }
        if self.Window.is_key_down(Key::C) { input.lock().unwrap().C = true; } else { input.lock().unwrap().C = false; }
        if self.Window.is_key_down(Key::D) { input.lock().unwrap().D = true; } else { input.lock().unwrap().D = false; }
        if self.Window.is_key_down(Key::E) { input.lock().unwrap().E = true; } else { input.lock().unwrap().E = false; }
        if self.Window.is_key_down(Key::F) { input.lock().unwrap().F = true; } else { input.lock().unwrap().F = false; }
        if self.Window.is_key_down(Key::G) { input.lock().unwrap().G = true; } else { input.lock().unwrap().G = false; }
        if self.Window.is_key_down(Key::H) { input.lock().unwrap().H = true; } else { input.lock().unwrap().H = false; }
        if self.Window.is_key_down(Key::I) { input.lock().unwrap().I = true; } else { input.lock().unwrap().I = false; }
        if self.Window.is_key_down(Key::J) { input.lock().unwrap().J = true; } else { input.lock().unwrap().J = false; }
        if self.Window.is_key_down(Key::K) { input.lock().unwrap().K = true; } else { input.lock().unwrap().K = false; }
        if self.Window.is_key_down(Key::L) { input.lock().unwrap().L = true; } else { input.lock().unwrap().L = false; }
        if self.Window.is_key_down(Key::M) { input.lock().unwrap().M = true; } else { input.lock().unwrap().M = false; }
        if self.Window.is_key_down(Key::N) { input.lock().unwrap().N = true; } else { input.lock().unwrap().N = false; }
        if self.Window.is_key_down(Key::O) { input.lock().unwrap().O = true; } else { input.lock().unwrap().O = false; }
        if self.Window.is_key_down(Key::P) { input.lock().unwrap().P = true; } else { input.lock().unwrap().P = false; }
        if self.Window.is_key_down(Key::Q) { input.lock().unwrap().Q = true; } else { input.lock().unwrap().Q = false; }
        if self.Window.is_key_down(Key::R) { input.lock().unwrap().R = true; } else { input.lock().unwrap().R = false; }
        if self.Window.is_key_down(Key::S) { input.lock().unwrap().S = true; } else { input.lock().unwrap().S = false; }
        if self.Window.is_key_down(Key::T) { input.lock().unwrap().T = true; } else { input.lock().unwrap().T = false; }
        if self.Window.is_key_down(Key::U) { input.lock().unwrap().U = true; } else { input.lock().unwrap().U = false; }
        if self.Window.is_key_down(Key::V) { input.lock().unwrap().V = true; } else { input.lock().unwrap().V = false; }
        if self.Window.is_key_down(Key::W) { input.lock().unwrap().W = true; } else { input.lock().unwrap().W = false; }
        if self.Window.is_key_down(Key::X) { input.lock().unwrap().X = true; } else { input.lock().unwrap().X = false; }
        if self.Window.is_key_down(Key::Y) { input.lock().unwrap().Y = true; } else { input.lock().unwrap().Y = false; }
        if self.Window.is_key_down(Key::Z) { input.lock().unwrap().Z = true; } else { input.lock().unwrap().Z = false; }
        if self.Window.is_key_down(Key::Up) { input.lock().unwrap().ArrowUp = true; } else { input.lock().unwrap().ArrowUp = false; }
        if self.Window.is_key_down(Key::Down) { input.lock().unwrap().ArrowDown = true; } else { input.lock().unwrap().ArrowDown = false; }
        if self.Window.is_key_down(Key::Left) { input.lock().unwrap().ArrowLeft = true; } else { input.lock().unwrap().ArrowLeft = false; }
        if self.Window.is_key_down(Key::Right) { input.lock().unwrap().ArrowRight = true; } else { input.lock().unwrap().ArrowRight = false; }
        if self.Window.is_key_down(Key::Key0) { input.lock().unwrap().Num0 = true; } else { input.lock().unwrap().Num0 = false; }
        if self.Window.is_key_down(Key::Key1) { input.lock().unwrap().Num1 = true; } else { input.lock().unwrap().Num1 = false; }
        if self.Window.is_key_down(Key::Key2) { input.lock().unwrap().Num2 = true; } else { input.lock().unwrap().Num2 = false; }
        if self.Window.is_key_down(Key::Key3) { input.lock().unwrap().Num3 = true; } else { input.lock().unwrap().Num3 = false; }
        if self.Window.is_key_down(Key::Key4) { input.lock().unwrap().Num4 = true; } else { input.lock().unwrap().Num4 = false; }
        if self.Window.is_key_down(Key::Key5) { input.lock().unwrap().Num5 = true; } else { input.lock().unwrap().Num5 = false; }
        if self.Window.is_key_down(Key::Key6) { input.lock().unwrap().Num6 = true; } else { input.lock().unwrap().Num6 = false; }
        if self.Window.is_key_down(Key::Key7) { input.lock().unwrap().Num7 = true; } else { input.lock().unwrap().Num7 = false; }
        if self.Window.is_key_down(Key::Key8) { input.lock().unwrap().Num8 = true; } else { input.lock().unwrap().Num8 = false; }
        if self.Window.is_key_down(Key::Key9) { input.lock().unwrap().Num9 = true; } else { input.lock().unwrap().Num9 = false; }
        if self.Window.is_key_down(Key::LeftCtrl) { input.lock().unwrap().Ctrl = true; } else { input.lock().unwrap().Ctrl = false; }
        if self.Window.is_key_down(Key::LeftAlt) { input.lock().unwrap().Alt = true; } else { input.lock().unwrap().Alt = false; }
        if self.Window.is_key_down(Key::LeftShift) { input.lock().unwrap().Shift = true; } else { input.lock().unwrap().Shift = false; }
        if self.Window.is_key_down(Key::CapsLock) { input.lock().unwrap().CapsLock = true; } else { input.lock().unwrap().CapsLock = false; }
        if self.Window.is_key_down(Key::Tab) { input.lock().unwrap().Tab = true; }
        if self.Window.is_key_down(Key::Escape) { input.lock().unwrap().Esc = true; } else { input.lock().unwrap().Esc = false; }
        if self.Window.is_key_down(Key::Space) { input.lock().unwrap().Space = true; } else { input.lock().unwrap().Space = false; }

    }
}

fn debug(max_update_time: Duration, work_time: Duration, sleep_time: Duration, debug: bool, debug_time: bool, title: String) -> String {
    if debug {
        let fps: String;
        if work_time.as_micros() > 0 { fps = format!("{}", 1_000_000 / max_update_time.as_micros()) }
        else { fps = "Approx. Infinity".to_string(); }
        if debug_time {
            let mut fre = 0; if max_update_time.as_micros() > work_time.as_micros() { fre = max_update_time.as_micros() - work_time.as_micros() }
            println!("MAX: {:>5}us; REAL: {:>5}us; FREE: {:>5}us, SLEEP: {:>5}us", max_update_time.as_micros(), work_time.as_micros(), fre, sleep_time.as_micros());
        }
        return format!("{}; {} FPS", title, fps);
    } else { return title; }
}
