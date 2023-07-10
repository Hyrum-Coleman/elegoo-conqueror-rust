#![no_std]
#![no_main]
mod hardware;
mod tools;

use tools::println;
use tools::CONSOLE;

use hardware::motor::{Motor};
use hardware::irmodule::IrModule;

use panic_halt as _;

use arduino_hal::simple_pwm::*;
use arduino_hal::delay_ms;
use avr_device::interrupt;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    tools::put_console(serial);

    let mut led = pins.d13.into_output();

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    let right_speed_pin = pins.d5.into_output().into_pwm(&timer0);
    let left_speed_pin = pins.d6.into_output().into_pwm(&timer0);
    let right_power_pin = pins.d7.into_output();
    let left_power_pin = pins.d8.into_output();
    let standby = pins.d3.into_output();

    let mut motor = Motor::new(
        right_speed_pin,
        left_speed_pin,
        right_power_pin,
        left_power_pin,
        standby,
    );
    motor.enable();
    let speed = 200;

    let ir_left = pins.a2.into_analog_input(&mut adc);
    let ir_middle = pins.a1.into_analog_input(&mut adc);
    let ir_rigt = pins.a0.into_analog_input(&mut adc);

    let ir_module = IrModule::new(ir_left, ir_middle, ir_rigt);

    loop {
        println!("Proving I am here");
        led.toggle();
        delay_ms(10);
        motor.drive(ir_module.get_direction(&mut adc), speed);
    }
}
