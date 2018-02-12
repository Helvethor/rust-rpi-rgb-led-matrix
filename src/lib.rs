extern crate libc;
mod c;

use libc::{c_int, c_char};
use std::ptr::null;
use std::ffi::CString;
use std::path::Path;
use std::ops::Deref;

pub use c::LedCanvas;
pub use c::LedMatrixOptions;
pub use c::LedColor;


pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    _options: LedMatrixOptions,
}

pub struct LedFont {
    handle: *mut c::LedFont
}


impl LedMatrix {
    pub fn new(options: Option<LedMatrixOptions>) -> Result<LedMatrix, &'static str> {
        let options = {
            if let Some(o) = options {
                o
            }
            else {
                LedMatrixOptions::new()
            }
        };

        let handle = unsafe {
            c::led_matrix_create_from_options(
                &options as *const LedMatrixOptions,
                null::<c_int>() as *mut c_int,
                null::<c_char>() as *mut*mut*mut c_char)
        };

        if handle.is_null() {
            Err("Couldn't create LedMatrix")
        }
        else {
            Ok(LedMatrix {
                handle,
                _options: options
            })
        }
    }

    pub fn canvas(&self) -> &mut LedCanvas {
        unsafe {
            &mut *c::led_matrix_get_canvas(self.handle)
        }
    }

    pub fn offscreen_canvas(&self) -> &mut LedCanvas {
        unsafe {
            &mut *c::led_matrix_create_offscreen_canvas(self.handle)
        }
    }

    pub fn swap(&self, canvas: &mut LedCanvas) -> &mut LedCanvas {
        unsafe {
            &mut *c::led_matrix_swap_on_vsync(self.handle, canvas)
        }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}


impl LedFont {
    pub fn new(bdf_file: &Path) -> Result<LedFont, &'static str> {
        let string = match bdf_file.to_str() {
            Some(s) => s,
            None => return Err("Couldn't convert path to str")
        };
        let cstring = CString::new(string).unwrap();

        let handle = unsafe {
            c::load_font(cstring.as_ptr())
        };

        if handle.is_null() {
            Err("Couldn't load font")
        }
        else {
            Ok(LedFont {
                handle
            })
        }
    }
}

impl Deref for LedFont {
    type Target = c::LedFont;
        
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.handle
        }
    }
}

impl Drop for LedFont {
    fn drop(&mut self) {
        unsafe {
            c::delete_font(self.handle)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use std::f64::consts::PI;

    fn led_matrix() -> LedMatrix {
        let mut options = LedMatrixOptions::new();
        options.set_hardware_mapping("adafruit-hat-pwm");
        options.set_chain_length(2);
        options.set_hardware_pulsing(false);
        //options.set_inverse_colors(true);
        //options.set_refresh_rate(true);
        LedMatrix::new(Some(options)).unwrap()
    }

    #[test]
    fn matrix_create() {
        let _matrix = led_matrix();
    }

    #[test]
    fn canvas_size() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        assert_eq!(canvas.size(), (64, 32));
    }

    #[test]
    fn draw_line() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        let (width, height) = canvas.size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0
        };

        canvas.clear();
        for x in 0..width {
            color.blue = 255 - 3 * x as u8;
            canvas.draw_line(x, 0, width - 1 - x, height - 1, &color);
            thread::sleep(time::Duration::new(0, 10000000));
        }
    }

    #[test]
    fn draw_circle() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        let (width, height) = canvas.size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0
        };
        let (x, y) = (width / 2, height / 2);

        canvas.clear();
        for r in 0..(width / 2) {
            color.green = color.red;
            color.red = color.blue;
            color.blue = (r * r) as u8;
            canvas.draw_circle(x, y, r as u32, &color);
            thread::sleep(time::Duration::new(0, 100000000));
        }
    }

    #[test]
    fn draw_text() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let font = LedFont::new(
            Path::new("/usr/share/fonts/misc/10x20.bdf")).unwrap();
        let color = LedColor {
            red: 0,
            green: 127,
            blue: 0
        };
        let (width, height) = canvas.size();
        let text_width = 10 * 9;
        let baseline = height / 2;

        canvas = matrix.offscreen_canvas();
        for x in 0..(2 * width) {
            let x = x % (10 * 9);
            canvas.clear();
            canvas.draw_text(&font, "Mah boy! ",
                x, baseline, &color, 0, false);
            canvas.draw_text(&font, "Mah boy! ",
                x - text_width, baseline, &color, 0, false);
            canvas.draw_text(&font, "Mah boy! ",
                x + text_width, baseline, &color, 0, false);
            canvas = matrix.swap(canvas);
            thread::sleep(time::Duration::new(0, 50000000));
        }
    }

    #[test]
    fn gradient() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        let mut color = LedColor {
            red: 0,
            green: 0,
            blue: 0
        };
        let period = 400;
        let duration = time::Duration::new(3, 0);
        let sleep_duration = duration / period;

        for t in 0..period {
            let t = t as f64;
            color.red = ((PI * t / period as f64).sin() * 255.) as u8;
            color.green = ((2. * PI * t / period as f64).cos() * 255.) as u8;
            color.blue = ((3. * PI * t / period as f64 + 0.3).cos() * 255.) as u8;
            canvas.fill(&color);
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    fn canvas_swap() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 127,
            green: 127,
            blue: 0
        };
        
        canvas.fill(&color);
        canvas = matrix.offscreen_canvas();
        color.blue = 127;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        canvas = matrix.swap(canvas);
        color.red = 0;
        canvas.fill(&color);
        thread::sleep(time::Duration::new(0, 500000000));
        matrix.swap(canvas);
        thread::sleep(time::Duration::new(0, 500000000));
    }
}
