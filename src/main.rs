#![no_std]
#![no_main]
mod tools;

use tools::println;
use tools::CONSOLE;

use panic_halt as _;

use avr_device::{interrupt};
type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    tools::put_console(serial);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        println!("Working print macro!!!");
        arduino_hal::delay_ms(1000);
    }
}
