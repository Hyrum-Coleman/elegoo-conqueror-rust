#![no_std]
#![no_main]
mod tools;
mod hardware;

use tools::println;
use tools::CONSOLE;

use hardware::motor::Motor;

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

    let right_speed_pin = pins.d5.into_output();
    let left_speed_pin = pins.d6.into_output();
    let right_power_pin = pins.d7.into_output();
    let left_power_pin = pins.d8.into_output();
    let standby = pins.d3.into_output();

    let mut motor = Motor::new(right_speed_pin, left_speed_pin, right_power_pin, left_power_pin, standby);
    loop {
        led.toggle();
        println!("Working print macro!!!");
        arduino_hal::delay_ms(1000);
    }
}
