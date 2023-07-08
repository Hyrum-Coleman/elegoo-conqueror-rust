#![no_std]
#![no_main]
mod tools;
mod hardware;

use tools::println;
use tools::CONSOLE;

use hardware::motor::{Motor, Direction};

use panic_halt as _;

use arduino_hal::simple_pwm::*;
use avr_device::{interrupt};

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

    tools::put_console(serial);

    let mut led = pins.d13.into_output();

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    let right_speed_pin = pins.d5.into_output().into_pwm(&timer0);
    let left_speed_pin = pins.d6.into_output().into_pwm(&timer0);
    let right_power_pin = pins.d7.into_output();
    let left_power_pin = pins.d8.into_output();
    let standby = pins.d3.into_output();

    let mut motor = Motor::new(right_speed_pin, left_speed_pin, right_power_pin, left_power_pin, standby);
    motor.enable();
    let speed = 200;

    loop {
        led.toggle();
        println!("Turning motor on: Going forwards");
        motor.drive(Direction::Forwards, speed);
        arduino_hal::delay_ms(1000);
        println!("Turning left");
        motor.drive(Direction::Left, speed);
        arduino_hal::delay_ms(1000);
        println!("Driving Backwards");
        motor.drive(Direction::Backwards, speed);
        arduino_hal::delay_ms(1000);
        println!("Turning Right");
        motor.drive(Direction::Right, speed);
        arduino_hal::delay_ms(1000);
        println!("Stopping motor");
        motor.stop();
        arduino_hal::delay_ms(1000);
        println!("Forward Left");
        motor.drive(Direction::ForwardsLeft, speed);
        arduino_hal::delay_ms(1000);
        println!("Backwards Right");
        motor.drive(Direction::BackwardsRight, speed);
        arduino_hal::delay_ms(1000);
        println!("Forwards Right");
        motor.drive(Direction::ForwardsRight, speed);
        arduino_hal::delay_ms(1000);
        println!("Backwards Left");
        motor.drive(Direction::BackwardsLeft, speed);
        arduino_hal::delay_ms(1000);
        println!("Stopping");
        motor.stop();
        arduino_hal::delay_ms(1000);
    }
}
