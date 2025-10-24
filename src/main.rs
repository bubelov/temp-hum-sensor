#![no_std]
#![no_main]

use crate::sht3x::Sht3x;
use core::panic::PanicInfo;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::logger::init_logger;
use log::{error, info};
mod sht3x;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    init_logger(log::LevelFilter::Debug);

    // hardware init
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // pins
    let led_pin = peripherals.GPIO15;
    let i2c_sda_pin = peripherals.GPIO22;
    let i2c_scl_pin = peripherals.GPIO23;

    info!("initializing led");
    let mut led = Output::new(led_pin, Level::Low, OutputConfig::default());
    info!("led init complete");

    info!("initializing i2c");
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("i2c init failed")
        .with_sda(i2c_sda_pin)
        .with_scl(i2c_scl_pin);
    info!("i2c init complete");

    let mut sht3x = Sht3x::new(i2c, Sht3x::DEFAULT_ADDR);

    loop {
        led.toggle();

        let measurement = sht3x.measure();
        info!(
            "temp: {} | hum: {}",
            measurement.temp_celsius, measurement.humidity_percent
        );

        // dev builds should use delay
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}

        // prod builds might try deep sleep to save energy
        //let mut rtc = Rtc::new(peripherals.LPWR);
        //rtc.sleep_deep(&[&TimerWakeupSource::new(Duration::from_millis(1000))]);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("*** PANIC ***");
    if let Some(location) = info.location() {
        error!("File: {}:{}", location.file(), location.line());
    }
    error!("Reason: {}", info.message());    
    loop {}
}
