//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO5)

#![no_std]
#![no_main]

// use esp_backtrace as _;
use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
    pac::Peripherals,
    prelude::*,
    system::SystemExt,
    timer::TimerGroup,
    Delay,
    Rtc,
};

use riscv_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT and the RTC WDT.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let mut timer0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer1 = TimerGroup::new(peripherals.TIMG1, &clocks);

    timer0.wdt.disable();
    timer1.wdt.disable();
    rtc.swd.disable();
    rtc.rwdt.disable();

    // Set GPIO5 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut red_led = io.pins.gpio5.into_push_pull_output();
    let mut blue_led = io.pins.gpio4.into_push_pull_output();

    red_led.set_high().unwrap();
    blue_led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a loop.
    let mut delay = Delay::new(&clocks);

    loop {
        red_led.toggle().unwrap();
        delay.delay_ms(500u32);
        red_led.toggle().unwrap();
        blue_led.toggle().unwrap();
        delay.delay_ms(500u32);
        blue_led.toggle().unwrap();
    }
}
