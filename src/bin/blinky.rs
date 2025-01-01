#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Hello world!");

    // Set GPIO2 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(1000u32);
    }
}