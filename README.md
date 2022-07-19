# Overview

This program uses rust to blink morse code on a raspberry pi pico led. I wrote this program to learn the basics of using rust in an embedded environment.

[Software Demo Video](https://youtu.be/ylfX56gYo_g)

# Development Environment

To develop this program I used Visual Studio Code with the rust-analyzer extension.

This program uses rust with the following dependencies:
* [rp2040-hal](https://crates.io/crates/rp2040-hal)
* [embedded-hal](https://crates.io/crates/embedded-hal)
* [embedded-time](https://crates.io/crates/embedded-time)
* [cortex-m-rt](https://crates.io/crates/cortex-m-rt)
* [panic-halt](https://crates.io/crates/panic-halt)
* [rp2040-boot2](https://crates.io/crates/rp2040-boot2)
* [cortex-m](https://crates.io/crates/cortex-m)

# Useful Websites

* [Rust Website](https://www.rust-lang.org/learn)
* [Crates io](https://crates.io/)
* [Tutorials Point](https://www.tutorialspoint.com/rust/index.htm)
* [rp-hal Github](https://github.com/rp-rs/rp-hal)

# Future Work

* Integrate keyboard or other input to control the led
* Integrate hardware to be controlled by the raspberry pi pico