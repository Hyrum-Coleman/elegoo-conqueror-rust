#![no_std]
#![no_main]

mod hardware;
mod tools;

use tools::println;
use tools::CONSOLE;

use hardware::motor::{Motor};
use hardware::sensors::{irmodule::IrModule, echo_module::SensorUnit};

use panic_halt as _;

use arduino_hal::simple_pwm::*;
use arduino_hal::delay_ms;
use avr_device::interrupt;
use crate::hardware::sensors::combined_sensor::CombinedSensor;
use crate::hardware::servo::ServoUnit;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    tools::put_console(serial);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut sensor_timer = dp.TC1;
    sensor_timer.tccr1b.write(|w| w.cs1().prescale_64());

    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale1024);

    let trig_pin = pins.d13.into_output();
    let echo_pin = pins.d12.into_floating_input();

    let sensor_unit = SensorUnit::new(trig_pin, echo_pin, sensor_timer);

    let ir_left = pins.a2.into_analog_input(&mut adc);
    let ir_middle = pins.a1.into_analog_input(&mut adc);
    let ir_right = pins.a0.into_analog_input(&mut adc);

    let ir_module = IrModule::new(ir_left, ir_middle, ir_right);

    let mut combined_sensor = CombinedSensor::new(ir_module, sensor_unit);

    let mut y_servo_pin = pins.d11.into_output().into_pwm(&mut timer2);
    y_servo_pin.enable();
    let mut y_servo = ServoUnit {
        servo: y_servo_pin
    };
    y_servo.look_right();

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

    loop {
        delay_ms(10);
        motor.drive(combined_sensor.get_direction(&mut adc), speed);
    }
}
