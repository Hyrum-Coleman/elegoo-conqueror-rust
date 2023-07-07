use arduino_hal::{port::{Pin, mode::Output}, hal::port::{PD5, PD6, PD7, PB0, PD3}};

pub struct Motor {
    pub right_speed_pin: Pin<Output, PD5>,
    pub left_speed_pin: Pin<Output, PD6>,
    pub right_power_pin: Pin<Output, PD7>,
    pub left_power_pin: Pin<Output, PB0>,
    pub standby_pin: Pin<Output, PD3>,
}

impl Motor {
    pub fn new(right_speed_pin: Pin<Output, PD5>, left_speed_pin: Pin<Output, PD6>, right_power_pin: Pin<Output, PD7>, left_power_pin: Pin<Output, PB0>, standby_pin: Pin<Output, PD3>) -> Self {
        Self { right_speed_pin, left_speed_pin, right_power_pin, left_power_pin, standby_pin }
    }

    pub fn drive_forwards(&mut self) {
        self.right_power_pin.set_high();
        self.left_power_pin.set_high();

        self.right_speed_pin.set_high();
        self.left_power_pin.set_high();
    }

    pub fn stop(&mut self) {
        self.right_power_pin.set_low();
        self.left_power_pin.set_low();
        self.right_speed_pin.set_low();
        self.left_speed_pin.set_low();
    }
}