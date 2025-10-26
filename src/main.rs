#![no_std]
#![no_main]

use crate::sht3x::Sht3x;
use core::cell::RefCell;
use core::panic::PanicInfo;
use embedded_hal_bus::i2c::RefCellDevice;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::logger::init_logger;
use log::{error, info};
mod sht3x;

#[cfg(feature = "display")]
use {
    embedded_graphics::draw_target::DrawTarget,
    embedded_graphics::pixelcolor::BinaryColor,
    ssd1306::mode::DisplayConfig,
    ssd1306::prelude::DisplayRotation,
    ssd1306::size::DisplaySize128x64,
    ssd1306::{I2CDisplayInterface, Ssd1306},
};

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
    let i2c_refcell = RefCell::new(i2c);
    info!("i2c init complete");

    let mut sht3x = Sht3x::new(RefCellDevice::new(&i2c_refcell), 0x44);

    #[cfg(feature = "display")]
    {
        let mut i2c_borrow = i2c_refcell.borrow_mut();
        let interface = I2CDisplayInterface::new(&mut *i2c_borrow);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();
        display.set_display_on(true).unwrap();
        display.clear(BinaryColor::On).unwrap();
        display.flush().unwrap();
    }

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
