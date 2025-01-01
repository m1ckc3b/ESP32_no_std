#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
  delay::Delay,
    gpio::{Event, Input, Io, Level, Output, Pull},
    prelude::*,
};
use esp_println::println;

static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Interrupt is ready!");

    let mut io = Io::new(peripherals.IO_MUX);
    // Set the interrupt handler for GPIO interrupts.
    io.set_interrupt_handler(handler);

    // Set GPIO2 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    // Set GPIO4 as an input
    let mut button = Input::new(peripherals.GPIO4, Pull::Up);


    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    let delay = Delay::new();

    loop {
      critical_section::with(|cs| {
        if let Some(ref button) = *BUTTON.borrow_ref(cs){
          if button.is_high() {
            led.toggle();
            delay.delay_millis(100);
          } else {
            led.set_low();
          }
        }
      });
    }
}

#[handler]
fn handler() {
    critical_section::with(|cs| {
        println!("GPIO interrupt");
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}