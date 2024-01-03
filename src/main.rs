#![no_std]
#![no_main]

use core::panic;

use esp_backtrace as _;
use defmt as _;
use esp32c6_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    panic!("Hello!");
    loop {
    }
}
