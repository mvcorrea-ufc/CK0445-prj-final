#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("ESP32-C3 MQTT Counter Project - Blink Test");

    // Set GPIO8 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO8, Level::Low);

    led.set_high();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a loop.
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
        println!("LED status: {:?}", led.output_level());
    }
}