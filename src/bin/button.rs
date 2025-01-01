#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Level, Output, Pull},
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Button is ready!");

    // Set GPIO2 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO2, Level::Low);
    let button = Input::new(peripherals.GPIO4, Pull::Up);

    // Check the button state and set the LED state accordingly.
    loop {
      if button.is_high() {
        led.set_high();
        println!("Button is pressed");
      } else {
        led.set_low();
      }
    }
}