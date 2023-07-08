use arduino_hal::{port::{Pin, mode::{Output, PwmOutput}}, hal::port::{PD5, PD6, PD7, PB0, PD3}, simple_pwm::Timer0Pwm};

pub struct Motor {
    pub right_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD5>,
    pub left_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD6>,
    pub right_power_pin: Pin<Output, PD7>,
    pub left_power_pin: Pin<Output, PB0>,
    pub standby_pin: Pin<Output, PD3>,
}

impl Motor {
    pub fn new(right_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD5>, left_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD6>, right_power_pin: Pin<Output, PD7>, left_power_pin: Pin<Output, PB0>, standby_pin: Pin<Output, PD3>) -> Self {
        Self { right_speed_pin, left_speed_pin, right_power_pin, left_power_pin, standby_pin }
    }

    pub fn enable(&mut self) {
        self.standby_pin.set_high();
        self.right_speed_pin.enable();
        self.left_speed_pin.enable();
    }

    pub fn drive(&mut self, direction: Direction, speed: u8) {
        match direction {
            Direction::Forwards => self.drive_forwards(speed),
            Direction::Backwards => self.drive_backwards(speed),
            Direction::Left => self.drive_left(speed),
            Direction::Right => self.drive_right(speed),
            Direction::ForwardsLeft => self.drive_forwards_left(speed),
            Direction::ForwardsRight => self.drive_forwards_right(speed),
            Direction::BackwardsLeft => self.drive_backwards_left(speed),
            Direction::BackwardsRight => self.drive_backwards_right(speed),
        }
    }

    pub fn stop(&mut self) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(0);
        self.left_speed_pin.set_duty(0);
    }

    fn drive_forwards(&mut self, speed: u8) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_high();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed);
    }

    fn drive_backwards(&mut self, speed: u8) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed);
    }

    fn drive_left(&mut self, speed: u8) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed);
    }

    fn drive_right(&mut self, speed: u8) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_high();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed);
    }

    fn drive_forwards_left(&mut self, speed: u8) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed - 50);
    }

    fn drive_forwards_right(&mut self, speed: u8) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_high();
        self.right_speed_pin.set_duty(speed - 50);
        self.left_speed_pin.set_duty(speed);
    }

    fn drive_backwards_left(&mut self, speed: u8) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_high();
        self.right_speed_pin.set_duty(speed);
        self.left_speed_pin.set_duty(speed - 50);
    }

    fn drive_backwards_right(&mut self, speed: u8) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(speed - 50);
        self.left_speed_pin.set_duty(speed);
    }
}

pub enum Direction {
    Forwards,
    Backwards,
    Left,
    Right,
    ForwardsLeft,
    ForwardsRight,
    BackwardsLeft,
    BackwardsRight,
}