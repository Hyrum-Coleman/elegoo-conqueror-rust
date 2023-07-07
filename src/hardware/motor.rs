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

    pub fn drive_forwards(&mut self) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_high();
        self.right_speed_pin.set_duty(144);
        self.left_speed_pin.set_duty(144);
    }

    pub fn stop(&mut self) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_duty(0);
        self.left_speed_pin.set_duty(0);
    }
}