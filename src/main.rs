#![no_std]
#![no_main]

use esp32c6_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let _clocks = ClockControl::max(system.clock_control).freeze();

    panic!("Hello!");
}
