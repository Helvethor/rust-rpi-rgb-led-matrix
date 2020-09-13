use libc::{c_char, c_int};
use std::ffi::CString;

type LedMatrixOptionsResult = Result<(), &'static str>;

/// The Rust representation of LedMatrixOptions, which contains parameters to specify your hardware setup.
#[derive(Debug)]
#[repr(C)]
pub struct LedMatrixOptions {
    pub(crate) hardware_mapping: *mut c_char,
    pub(crate) rows: c_int,
    pub(crate) cols: c_int,
    pub(crate) chain_length: c_int,
    pub(crate) parallel: c_int,
    pub(crate) pwm_bits: c_int,
    pub(crate) pwm_lsb_nanoseconds: c_int,
    pub(crate) pwm_dither_bits: c_int,
    pub(crate) brightness: c_int,
    pub(crate) scan_mode: c_int,
    pub(crate) row_address_type: c_int,
    pub(crate) multiplexing: c_int,
    pub(crate) led_rgb_sequence: *mut c_char,
    pub(crate) pixel_mapper_config: *mut c_char,
    pub(crate) panel_type: *mut c_char,
    pub(crate) disable_hardware_pulsing: c_char,
    pub(crate) show_refresh_rate: c_char,
    pub(crate) inverse_colors: c_char,
    pub(crate) limit_refresh_rate_hz: c_int,
}

/// The Rust representation of LedRuntimeOptions, which contains parameters to specify how the library behaves at runtime.
#[derive(Debug)]
#[repr(C)]
pub struct LedRuntimeOptions {
    pub(crate) gpio_slowdown: c_int,
    pub(crate) daemon: c_int,
    pub(crate) drop_privileges: c_int,
    pub(crate) do_gpio_init: bool,
}

impl LedMatrixOptions {
    /// Creates a new `LedMatrixOptions` struct with the default parameters.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix,LedMatrixOptions};
    /// let mut options = LedMatrixOptions::new();
    /// options.set_hardware_mapping("adafruit-hat-pwm");
    /// let matrix = LedMatrix::new(Some(options), None).unwrap();
    /// ```
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

    /// Sets the type of GPIO mapping used (e.g., "adafruit-hat-pwm").
    pub fn set_hardware_mapping(&mut self, mapping: &str) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            self.hardware_mapping = CString::new(mapping).unwrap().into_raw();
        }
    }

    /// Sets the number of rows on the panels being used. Typically 8, 16, 32 or 64.
    pub fn set_rows(&mut self, rows: u32) {
        self.rows = rows as c_int;
    }

    /// Sets the number of columns on the panels being used. Typically 32 or 64.
    pub fn set_cols(&mut self, cols: u32) {
        self.cols = cols as c_int;
    }

    /// Sets the number of panels daisy-chained together.
    pub fn set_chain_length(&mut self, chain_length: u32) {
        self.chain_length = chain_length as c_int;
    }

    /// Sets the number of parallel chains. Valid range: [1,3].
    pub fn set_parallel(&mut self, parallel: u32) {
        self.parallel = parallel as c_int;
    }

    /// Sets the number of PWM bits to use. Valid range: [1,11].
    pub fn set_pwm_bits(&mut self, pwm_bits: u8) -> LedMatrixOptionsResult {
        if pwm_bits > 11 {
            Err("Pwm bits can only have value between 0 and 11 inclusive")
        } else {
            self.pwm_bits = pwm_bits as c_int;
            Ok(())
        }
    }

    /// Sets the number of nanoseconds of delay for our LSB
    pub fn set_pwm_lsb_nanoseconds(&mut self, pwm_lsb_nanoseconds: u32) {
        self.pwm_lsb_nanoseconds = pwm_lsb_nanoseconds as c_int;
    }

    /// Sets the pannel brightness in percent.
    pub fn set_brightness(&mut self, brightness: u8) -> LedMatrixOptionsResult {
        if brightness > 100 || brightness < 1 {
            Err("Brigthness can only have value between 1 and 100 inclusive")
        } else {
            self.brightness = brightness as c_int;
            Ok(())
        }
    }

    /// Sets the scan mode. 0: progressive, 1: interlaced.
    pub fn set_scan_mode(&mut self, scan_mode: u32) {
        self.scan_mode = scan_mode as c_int;
    }

    /// Sets the ordering of the LEDs on your panel.
    pub fn set_led_rgb_sequence(&mut self, sequence: &str) {
        unsafe {
            let _ = CString::from_raw(self.led_rgb_sequence);
            self.led_rgb_sequence = CString::new(sequence).unwrap().into_raw();
        }
    }

    /// Semicolon-separated list of pixel-mappers to arrange pixels (e.g. "U-mapper;Rotate:90").
    ///
    /// Valid mapping options
    ///
    /// * Mirror
    /// * Rotate:<Angle>
    /// * U-mapper
    /// * V-mapper
    pub fn set_pixel_mapper_config(&mut self, mapper: &str) {
        unsafe {
            let _ = CString::from_raw(self.pixel_mapper_config);
            self.pixel_mapper_config = CString::new(mapper).unwrap().into_raw();
        }
    }

    /// Sets if hardware pin-pulse generation should be used.
    pub fn set_hardware_pulsing(&mut self, enable: bool) {
        if enable {
            self.disable_hardware_pulsing = 0;
        } else {
            self.disable_hardware_pulsing = 1;
        }
    }

    /// Configures if the current refresh rate should be printed by the C++ library.
    pub fn set_refresh_rate(&mut self, enable: bool) {
        if enable {
            self.show_refresh_rate = 1;
        } else {
            self.show_refresh_rate = 0;
        }
    }

    /// If set, invert the color displayed.
    pub fn set_inverse_colors(&mut self, enable: bool) {
        if enable {
            self.inverse_colors = 1;
        } else {
            self.inverse_colors = 0;
        }
    }

    /// Sets the type of multiplexing used.
    ///
    /// 0.  direct
    /// 1.  Stripe
    /// 2.  Checkered
    /// 3.  Spiral
    /// 4.  ZStripe
    /// 5.  ZnMirrorZStripe
    /// 6.  coreman
    /// 7.  Kaler2Scan
    /// 8.  ZStripeUneven
    /// 9.  P10-128x4-Z
    /// 10. QiangLiQ8
    /// 11. InversedZStripe
    /// 12. P10Outdoor1R1G1-1
    /// 13. P10Outdoor1R1G1-2
    /// 14. P10Outdoor1R1G1-3
    /// 15. P10CoremanMapper
    /// 16. P8Outdoor1R1G1
    pub fn set_multiplexing(&mut self, multiplexing: u32) {
        self.multiplexing = multiplexing as c_int;
    }

    /// Sets the type of row addressing to be used.
    ///
    /// 0. default
    /// 1. AB-addressed panels
    /// 2. direct row select
    /// 3. ABC-addressed panels
    /// 4. ABC Shift + DE direct
    pub fn set_row_addr_type(&mut self, row_addr_type: u32) {
        self.row_address_type = row_addr_type as c_int;
    }

    /// Limit refresh rate to this frequency in Hz. (0 = no limit)
    ///
    /// Useful to keep a constant refresh rate on loaded system.
    pub fn set_limit_refresh(&mut self, limit_refresh: u32) {
        self.limit_refresh_rate_hz = limit_refresh as c_int;
    }

    /// Configures how many bits to use for time-based dithering.
    pub fn set_pwm_dither_bits(&mut self, pwm_dither_bits: u32) {
        self.pwm_dither_bits = pwm_dither_bits as c_int;
    }

    /// Needed to initialize special panels. Supported: 'FM6126A', 'FM6127'
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
    /// Creates a new `LedRuntimeOptions` struct with the default parameters.
    ///
    /// ```
    /// use rpi_led_matrix::{LedMatrix, LedRuntimeOptions};
    /// let mut rt_options = LedRuntimeOptions::new();
    /// rt_options.set_gpio_slowdown(3);
    /// let matrix = LedMatrix::new(None, Some(rt_options)).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            gpio_slowdown: 1,
            daemon: 0,
            drop_privileges: 1,
            do_gpio_init: true,
        }
    }

    /// Sets the GPIO slowdown, in . Needed for faster Pis/slower panels
    pub fn set_gpio_slowdown(&mut self, gpio_slowdown: u32) {
        self.gpio_slowdown = gpio_slowdown as i32;
    }

    /// If True, make the process run in the background as daemon.
    pub fn set_daemon(&mut self, daemon: bool) {
        self.daemon = if daemon { 1 } else { 0 };
    }

    /// If True, drop privileges from 'root' after initializing the hardware.
    pub fn set_drop_privileges(&mut self, drop_privileges: bool) {
        self.drop_privileges = if drop_privileges { 1 } else { 0 };
    }

    /// You almost definitely want this to be left as True. Use this if you know what you're doing.
    pub fn set_do_gpio_init(&mut self, do_gpio_init: bool) {
        self.do_gpio_init = do_gpio_init;
    }
}

impl Default for LedRuntimeOptions {
    fn default() -> Self {
        Self::new()
    }
}
