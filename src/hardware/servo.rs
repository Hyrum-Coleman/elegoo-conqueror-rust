use arduino_hal::hal::port::PB3;
use arduino_hal::port::mode::PwmOutput;
use arduino_hal::port::Pin;
use arduino_hal::simple_pwm::Timer2Pwm;

const SERVO_CENTER: u8 = 23;
const SERVO_RIGHT: u8 = 15;
const SERVO_LEFT: u8 = 31;

pub struct ServoUnit {
    pub servo: Pin<PwmOutput<Timer2Pwm>, PB3>
}

impl ServoUnit {
    pub fn look_right(&mut self) {
        self.servo.set_duty(SERVO_RIGHT);
    }
    pub fn look_left(&mut self) {
        self.servo.set_duty(SERVO_LEFT);
    }
    pub fn look_front(&mut self) {
        self.servo.set_duty(SERVO_CENTER);
    }
}