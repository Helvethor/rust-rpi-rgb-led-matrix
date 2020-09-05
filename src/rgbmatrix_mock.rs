use crate::c;
use libc::{c_char, c_int};
use std::ptr;

pub(crate) extern "C" fn led_matrix_create_from_options(
    _options: *const c::LedMatrixOptions,
    _argc: *mut c_int,
    _argv: *mut *mut *mut c_char,
) -> *mut c::LedMatrix {
    ptr::null_mut::<c::LedMatrix>()
}

pub(crate) extern "C" fn led_matrix_delete(_matrix: *mut c::LedMatrix) {}

pub(crate) extern "C" fn led_matrix_get_canvas(_matrix: *mut c::LedMatrix) -> *mut c::LedCanvas {
    ptr::null_mut::<c::LedCanvas>()
}

pub(crate) extern "C" fn led_canvas_get_size(
    _canvas: *const c::LedCanvas,
    _width: *mut c_int,
    _height: *mut c_int,
) {
}

pub(crate) extern "C" fn led_canvas_set_pixel(
    _canvas: *mut c::LedCanvas,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

pub(crate) extern "C" fn led_canvas_clear(_canvas: *mut c::LedCanvas) {}

pub(crate) extern "C" fn led_canvas_fill(_canvas: *mut c::LedCanvas, _r: u8, _g: u8, _b: u8) {}

pub(crate) extern "C" fn led_matrix_create_offscreen_canvas(
    _matrix: *mut c::LedMatrix,
) -> *mut c::LedCanvas {
    ptr::null_mut::<c::LedCanvas>()
}

pub(crate) extern "C" fn led_matrix_swap_on_vsync(
    _matrix: *mut c::LedMatrix,
    _canvas: *mut c::LedCanvas,
) -> *mut c::LedCanvas {
    ptr::null_mut::<c::LedCanvas>()
}

pub(crate) extern "C" fn load_font(_bdf_font_file: *const c_char) -> *mut c::LedFont {
    ptr::null_mut::<c::LedFont>()
}

pub(crate) extern "C" fn delete_font(_font: *mut c::LedFont) {}

pub(crate) extern "C" fn draw_text(
    _canvas: *mut c::LedCanvas,
    _font: *const c::LedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    let value: c_int = 0;
    value
}

pub(crate) extern "C" fn vertical_draw_text(
    _canvas: *mut c::LedCanvas,
    _font: *const c::LedFont,
    _x: c_int,
    _y: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
    _utf8_text: *const c_char,
    _kerning_offset: c_int,
) -> c_int {
    let value: c_int = 0;
    value
}

pub(crate) extern "C" fn draw_circle(
    _canvas: *mut c::LedCanvas,
    _x: c_int,
    _y: c_int,
    _radius: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}

pub(crate) extern "C" fn draw_line(
    _canvas: *mut c::LedCanvas,
    _x0: c_int,
    _y0: c_int,
    _x1: c_int,
    _y1: c_int,
    _r: u8,
    _g: u8,
    _b: u8,
) {
}
