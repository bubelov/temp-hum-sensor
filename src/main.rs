#![no_std]
#![no_main]

use crate::sht3x::Sht3x;
use core::cell::RefCell;
use core::panic::PanicInfo;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_9X18_BOLD;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::Rectangle;
use embedded_hal_bus::i2c::RefCellDevice;
use embedded_text::TextBox;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::style::{HeightMode, TextBoxStyleBuilder};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::logger::init_logger;
use heapless::String;
use log::{error, info};
mod sht3x;
use core::fmt::Write;
use embedded_graphics::Drawable;
use {
    embedded_graphics::draw_target::DrawTarget,
    embedded_graphics::pixelcolor::BinaryColor,
    ssd1306::mode::DisplayConfig,
    ssd1306::prelude::DisplayRotation,
    ssd1306::size::DisplaySize128x32,
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

    let display_interface = I2CDisplayInterface::new(RefCellDevice::new(&i2c_refcell));
    let mut display = Ssd1306::new(
        display_interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    let text_style = MonoTextStyle::new(&FONT_9X18_BOLD, BinaryColor::On);

    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        .height_mode(HeightMode::FitToText)
        .build();

    display.init().unwrap();
    display.set_display_on(true).unwrap();
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    let mut buffer: String<32> = String::new();

    loop {
        led.toggle();

        let measurement = sht3x.measure();
        info!(
            "temp: {} | hum: {}",
            measurement.temp_celsius, measurement.humidity_percent
        );

        buffer.clear();
        write!(
            buffer,
            "{:.1}C\n{:.1}%",
            measurement.temp_celsius, measurement.humidity_percent
        )
        .unwrap();

        display.clear(BinaryColor::Off).unwrap();
        display.flush().unwrap();
        TextBox::with_textbox_style(
            &buffer,
            Rectangle::new(Point::new(0, 0), Size::new(128, 32)),
            text_style,
            textbox_style,
        )
        .draw(&mut display)
        .unwrap();
        display.flush().unwrap();

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
