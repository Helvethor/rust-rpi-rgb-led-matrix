# Control RGB LED displays from Rust

This repository contains rust bindings for [rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix).
In order to take advantage of the newer APIs, the minimum supported commit of the library is
[55fa32f](https://github.com/hzeller/rpi-rgb-led-matrix/commit/55fa32fc2e02afb254ac834aea93589d5b891a11)
(Sept. 7th, 2020), which exposes all parameters that affect how the matrix runs.
