use libc::{c_char, c_int};
use std::ffi::CString;

type LedMatrixOptionsResult = Result<(), &'static str>;

#[derive(Debug)]
#[repr(C)]
pub struct LedMatrixOptions {
    hardware_mapping: *mut c_char,
    rows: c_int,
    cols: c_int,
    chain_length: c_int,
    parallel: c_int,
    pwm_bits: c_int,
    pwm_lsb_nanoseconds: c_int,
    pwm_dither_bits: c_int,
    brightness: c_int,
    scan_mode: c_int,
    row_address_type: c_int,
    multiplexing: c_int,
    led_rgb_sequence: *mut c_char,
    pixel_mapper_config: *mut c_char,
    panel_type: *mut c_char,
    disable_hardware_pulsing: c_char,
    show_refresh_rate: c_char,
    inverse_colors: c_char,
    limit_refresh_rate_hz: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct LedRuntimeOptions {
    gpio_slowdown: c_int,
    daemon: c_int,
    drop_privileges: c_int,
    do_gpio_init: bool,
}

impl LedMatrixOptions {
    pub fn new() -> LedMatrixOptions {
        LedMatrixOptions {
            hardware_mapping: CString::new("regular").unwrap().into_raw(),
            rows: 32,
            cols: 32,
            chain_length: 1,
            parallel: 1,
            pwm_bits: 11,
            pwm_lsb_nanoseconds: 1000,
            pwm_dither_bits: 1,
            brightness: 100,
            scan_mode: 0,
            row_address_type: 0,
            multiplexing: 0,
            led_rgb_sequence: CString::new("RGB").unwrap().into_raw(),
            pixel_mapper_config: CString::new("").unwrap().into_raw(),
            panel_type: CString::new("").unwrap().into_raw(),
            disable_hardware_pulsing: 1,
            show_refresh_rate: 1,
            inverse_colors: 0,
            limit_refresh_rate_hz: 0,
        }
    }

    pub fn set_hardware_mapping(&mut self, mapping: &str) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            self.hardware_mapping = CString::new(mapping).unwrap().into_raw();
        }
    }

    pub fn set_rows(&mut self, rows: u32) {
        self.rows = rows as c_int;
    }

    pub fn set_cols(&mut self, cols: u32) {
        self.cols = cols as c_int;
    }

    pub fn set_chain_length(&mut self, chain_length: u32) {
        self.chain_length = chain_length as c_int;
    }

    pub fn set_parallel(&mut self, parallel: bool) {
        if parallel {
            self.parallel = 1;
        } else {
            self.parallel = 0;
        }
    }

    pub fn set_pwm_bits(&mut self, pwm_bits: u8) -> LedMatrixOptionsResult {
        if pwm_bits > 11 {
            Err("Pwm bits can only have value between 0 and 11 inclusive")
        } else {
            self.pwm_bits = pwm_bits as c_int;
            Ok(())
        }
    }

    pub fn set_pwm_lsb_nanoseconds(&mut self, pwm_lsb_nanoseconds: u32) {
        self.pwm_lsb_nanoseconds = pwm_lsb_nanoseconds as c_int;
    }

    pub fn set_brightness(&mut self, brightness: u8) -> LedMatrixOptionsResult {
        if brightness > 100 || brightness < 1 {
            Err("Brigthness can only have value between 1 and 100 inclusive")
        } else {
            self.brightness = brightness as c_int;
            Ok(())
        }
    }

    pub fn set_scan_mode(&mut self, scan_mode: bool) {
        if scan_mode {
            self.scan_mode = 1 as c_int;
        } else {
            self.scan_mode = 0 as c_int;
        }
    }

    pub fn set_led_rgb_sequence(&mut self, sequence: &str) {
        unsafe {
            let _ = CString::from_raw(self.led_rgb_sequence);
            self.led_rgb_sequence = CString::new(sequence).unwrap().into_raw();
        }
    }

    pub fn set_pixel_mapper_config(&mut self, mapper: &str) {
        unsafe {
            let _ = CString::from_raw(self.pixel_mapper_config);
            self.pixel_mapper_config = CString::new(mapper).unwrap().into_raw();
        }
    }

    pub fn set_hardware_pulsing(&mut self, enable: bool) {
        if enable {
            self.disable_hardware_pulsing = 0;
        } else {
            self.disable_hardware_pulsing = 1;
        }
    }

    pub fn set_refresh_rate(&mut self, enable: bool) {
        if enable {
            self.show_refresh_rate = 1;
        } else {
            self.show_refresh_rate = 0;
        }
    }

    pub fn set_inverse_colors(&mut self, enable: bool) {
        if enable {
            self.inverse_colors = 1;
        } else {
            self.inverse_colors = 0;
        }
    }

    pub fn set_multiplexing(&mut self, multiplexing: u32) {
        self.multiplexing = multiplexing as c_int;
    }

    pub fn set_row_addr_type(&mut self, row_addr_type: u32) {
        self.row_address_type = row_addr_type as c_int;
    }

    pub fn set_limit_refresh(&mut self, limit_refresh: u32) {
        self.limit_refresh_rate_hz = limit_refresh as c_int;
    }

    pub fn set_pwm_dither_bits(&mut self, pwm_dither_bits: u32) {
        self.pwm_dither_bits = pwm_dither_bits as c_int;
    }

    pub fn set_panel_type(&mut self, panel_type: &str) {
        unsafe {
            let _ = CString::from_raw(self.panel_type);
            self.panel_type = CString::new(panel_type).unwrap().into_raw();
        }
    }
}

impl Default for LedMatrixOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LedMatrixOptions {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            let _ = CString::from_raw(self.led_rgb_sequence);
            let _ = CString::from_raw(self.panel_type);
        }
    }
}

impl LedRuntimeOptions {
    pub fn new() -> Self {
        Self {
            gpio_slowdown: 1,
            daemon: 0,
            drop_privileges: 1,
            do_gpio_init: true,
        }
    }

    pub fn set_gpio_slowdown(&mut self, gpio_slowdown: i32) {
        self.gpio_slowdown = gpio_slowdown;
    }

    pub fn set_daemon(&mut self, daemon: i32) {
        self.daemon = daemon;
    }

    pub fn set_drop_privileges(&mut self, drop_privileges: i32) {
        self.drop_privileges = drop_privileges;
    }

    pub fn set_do_gpio_init(&mut self, do_gpio_init: bool) {
        self.do_gpio_init = do_gpio_init;
    }
}

impl Default for LedRuntimeOptions {
    fn default() -> Self {
        Self::new()
    }
}