use libc::c_int;
use std::ffi::CString;

use crate::c;
use crate::{LedColor, LedFont};

/// The Rust handle for the matrix canvas to draw on.
///
/// ```
/// use rpi_led_matrix::{LedMatrix, LedColor};
/// let matrix = LedMatrix::new(None, None).unwrap();
/// let canvas = matrix.canvas();
/// canvas.fill(&LedColor { red: 128, green: 128, blue: 128 });
/// ```
pub struct LedCanvas {
    pub(crate) handle: *mut c::LedCanvas,
}

impl LedCanvas {
    /// Retrieves the width & height of the canvas
    pub fn canvas_size(&self) -> (i32, i32) {
        let (mut width, mut height): (c_int, c_int) = (0, 0);
        unsafe {
            c::led_canvas_get_size(
                self.handle,
                &mut width as *mut c_int,
                &mut height as *mut c_int,
            );
        }
        (width as i32, height as i32)
    }

    /// Sets the pixel at the given coordinate to the given color.
    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            c::led_canvas_set_pixel(
                self.handle,
                x as c_int,
                y as c_int,
                color.red,
                color.green,
                color.blue,
            )
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        unsafe {
            c::led_canvas_clear(self.handle);
        }
    }

    /// Fills the canvas with the given color.
    pub fn fill(&mut self, color: &LedColor) {
        unsafe {
            c::led_canvas_fill(self.handle, color.red, color.green, color.blue);
        }
    }

    /// Draws a straight, one pixel wide line using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        unsafe {
            c::draw_line(
                self.handle,
                x0,
                y0,
                x1,
                y1,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    /// Draws a one pixel wide circle using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        unsafe {
            c::draw_circle(
                self.handle,
                x as c_int,
                y as c_int,
                radius as c_int,
                color.red,
                color.green,
                color.blue,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Renders text using the C++ library.
    pub fn draw_text(
        &mut self,
        font: &LedFont,
        text: &str,
        x: i32,
        y: i32,
        color: &LedColor,
        kerning_offset: i32,
        vertical: bool,
    ) -> i32 {
        let ctext = CString::new(text).unwrap();
        unsafe {
            if vertical {
                c::vertical_draw_text(
                    self.handle,
                    font.handle,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            } else {
                c::draw_text(
                    self.handle,
                    font.handle,
                    x as c_int,
                    y as c_int,
                    color.red,
                    color.green,
                    color.blue,
                    ctext.as_ptr(),
                    kerning_offset as c_int,
                ) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
    use std::f64::consts::PI;
    use std::{thread, time};

    fn led_matrix() -> LedMatrix {
        let mut options = LedMatrixOptions::new();
        let mut rt_options = LedRuntimeOptions::new();
        options.set_hardware_mapping("adafruit-hat-pwm");
        options.set_chain_length(2);
        options.set_hardware_pulsing(false);
        options.set_refresh_rate(true);
        options.set_brightness(10).unwrap();
        rt_options.set_gpio_slowdown(2);
        LedMatrix::new(Some(options), Some(rt_options)).unwrap()
    }

    #[test]
    fn size() {
        let matrix = led_matrix();
        let canvas = matrix.canvas();
        assert_eq!(canvas.canvas_size(), (64, 32));
    }

    #[test]
    fn draw_line() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
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
        let mut canvas = matrix.canvas();
        let (width, height) = canvas.canvas_size();
        let mut color = LedColor {
            red: 127,
            green: 0,
            blue: 0,
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
    fn gradient() {
        let matrix = led_matrix();
        let mut canvas = matrix.canvas();
        let mut color = LedColor {
            red: 0,
            green: 0,
            blue: 0,
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
            blue: 0,
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
