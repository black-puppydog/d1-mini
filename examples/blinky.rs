#![no_std]
#![no_main]

use panic_halt as _;
use xtensa_lx106_rt as _;

use d1_mini::{esp8266, esp8266_hal, Pins};
use esp8266::Peripherals;
use esp8266_hal::ehal::digital::v2::{OutputPin, ToggleableOutputPin};
use esp8266_hal::ehal::timer::CountDown;
use esp8266_hal::timer::{Nanoseconds, TimerExt};

const CORE_HZ: u32 = 80_000_000;

#[no_mangle]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let mut led = pins.d4.into_push_pull_output();
    led.set_high().unwrap();

    let (mut timer1, _) = peripherals.TIMER.timers(CORE_HZ);
    timer1.start(Nanoseconds(100_000_000));

    loop {
        nb::block!(timer1.wait()).unwrap();
        led.toggle().unwrap();
    }
}