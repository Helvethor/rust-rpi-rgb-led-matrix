# rpi-led-matrix

Safe rust bindings on top of the C++ library [`rpi-rgb-led-matrix`].

## Usage

The examples have more detailed usage, but here is basic usage to get things rendering on your display.
```rust
use rpi_led_matrix::{LedMatrix, LedColor};

let matrix = LedMatrix::new(None, None).unwrap();
let mut canvas = matrix.offscreen_canvas();
for red in 0..255 {
    for green in 0..255 {
        for blue in 0..255 {
            canvas.fill(&LedColor { red, green, blue });
            canvas = matrix.swap(canvas);
        }
    }
}
```
Note that if you have wirings other than the libraries "default", you will need to construct arguments to the library to specify the layout. See `LedMatrixOptions` for more information.

## [Documentation](https://docs.rs/rpi-led-matrix/)

[`rpi-rgb-led-matrix`]: https://github.com/hzeller/rpi-rgb-led-matrix
