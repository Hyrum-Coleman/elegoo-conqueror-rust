use arduino_hal::{
    hal::port::{PB0, PD3, PD5, PD6, PD7},
    port::{
        mode::{Output, PwmOutput},
        Pin,
    },
    simple_pwm::Timer0Pwm,
};

pub struct Motor {
    pub right_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD5>,
    pub left_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD6>,
    pub right_power_pin: Pin<Output, PD7>,
    pub left_power_pin: Pin<Output, PB0>,
    pub standby_pin: Pin<Output, PD3>,
    pub last_direction: Direction,
}

impl Motor {
    pub fn new(
        right_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD5>,
        left_speed_pin: Pin<PwmOutput<Timer0Pwm>, PD6>,
        right_power_pin: Pin<Output, PD7>,
        left_power_pin: Pin<Output, PB0>,
        standby_pin: Pin<Output, PD3>,
    ) -> Self {
        Self {
            right_speed_pin,
            left_speed_pin,
            right_power_pin,
            left_power_pin,
            standby_pin,
            last_direction: Direction::None,
        }
    }

    pub fn enable(&mut self) {
        self.standby_pin.set_high();
        self.right_speed_pin.enable();
        self.left_speed_pin.enable();
    }

    pub fn drive(&mut self, direction: Direction, speed: u8) {

        if direction == self.last_direction {
            return
        }

        match direction {
            Direction::Forwards => self.drive_forwards(speed),
            Direction::Backwards => self.drive_backwards(speed),
            Direction::Left => self.drive_left(speed),
            Direction::Right => self.drive_right(speed),
            Direction::None => self.stop(),
        }
        self.last_direction = direction;
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
}

#[derive(PartialEq)]
pub enum Direction {
    Forwards,
    Backwards,
    Left,
    Right,
    None,
}
