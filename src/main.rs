#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_hal::prelude::*;
use arduino_hal::port::mode::Output;
use arduino_hal::delay::Delay;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
