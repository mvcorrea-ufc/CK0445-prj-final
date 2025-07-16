#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
    timer::timg::TimerGroup,
};
use esp_wifi::{initialize, EspWifiInitFor};

mod config;
mod wifi;

use config::*;
use wifi::*;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("ESP32-C3 MQTT Counter Project - WiFi Test");

    // Set GPIO8 as an output for status indication
    let mut led = Output::new(peripherals.GPIO8, Level::Low);
    led.set_high();

    // Initialize the Delay peripheral
    let delay = Delay::new();

    // Initialize timer for WiFi
    let timer_group_0 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init = initialize(
        EspWifiInitFor::Wifi,
        timer_group_0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // Initialize WiFi
    let (wifi_device, _) = esp_wifi::wifi::new_with_mode(&wifi_init, peripherals.WIFI, esp_wifi::wifi::WifiStaDevice).unwrap();
    
    let mut wifi_manager = init_wifi(wifi_init, wifi_device).unwrap();

    println!("WiFi: Starting connection process...");
    
    // Try to connect to WiFi
    match wifi_manager.connect() {
        Ok(_) => {
            println!("WiFi: Connected successfully!");
            // Indicate success with faster blinking
            for _ in 0..10 {
                led.toggle();
                delay.delay_millis(100);
            }
        }
        Err(e) => {
            println!("WiFi: Connection failed: {:?}", e);
            // Indicate failure with slow blinking
            for _ in 0..5 {
                led.toggle();
                delay.delay_millis(1000);
            }
        }
    }

    // Main loop - monitor WiFi status
    loop {
        if wifi_manager.is_connected() {
            // Connected - slow blink
            led.set_high();
            delay.delay_millis(1800);
            led.set_low();
            delay.delay_millis(200);
        } else {
            // Disconnected - fast blink
            led.toggle();
            delay.delay_millis(100);
            
            // Try to reconnect every 10 seconds
            static mut RECONNECT_COUNTER: u32 = 0;
            unsafe {
                RECONNECT_COUNTER += 1;
                if RECONNECT_COUNTER >= 100 { // 100 * 100ms = 10s
                    RECONNECT_COUNTER = 0;
                    println!("WiFi: Attempting reconnection...");
                    if let Err(e) = wifi_manager.reconnect() {
                        println!("WiFi: Reconnection failed: {:?}", e);
                    }
                }
            }
        }
    }
}