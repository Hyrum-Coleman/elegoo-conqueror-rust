use arduino_hal::{port::{Pin, mode::Analog}, hal::port::{PC2, PC1, PC0}, Adc};

use crate::hardware::motor::Direction;

pub struct IrModule {
    left_sensor: Pin<Analog, PC2>,
    middle_sensor: Pin<Analog, PC1>,
    right_sensor: Pin<Analog, PC0>,
}

impl IrModule {
    pub fn new(left_sensor: Pin<Analog, PC2>, middle_sensor: Pin<Analog, PC1>, right_sensor: Pin<Analog, PC0>) -> Self {
        Self { left_sensor, middle_sensor, right_sensor }
    }

    pub fn get_direction(&self, adc: &mut Adc) -> Direction {
        if !self.is_car_on_ground(adc) {
            return Direction::None;
        }

        if self.is_middle_on_line(adc) {
            return Direction::Forwards;
        }

        if self.is_right_on_line(adc) {
            return Direction::Right;
        }

        if self.is_left_on_line(adc) {
            return Direction::Left;
        }
        self.car_off_line(adc)
    }

    pub fn is_car_on_ground(&self, adc: &mut Adc) -> bool {
        if self.right_sensor.analog_read(adc) > 950 && self.middle_sensor.analog_read(adc) > 950 && self.left_sensor.analog_read(adc) > 950 {
            return false;
        }
        true
    }

    fn is_middle_on_line(&self, adc: &mut Adc) -> bool {
        match self.middle_sensor.analog_read(adc) {
            0..=250 => false,
            251..=950 => true,
            951.. => false,
        }
    }

    fn is_right_on_line(&self, adc: &mut Adc) -> bool {
        match self.right_sensor.analog_read(adc) {
            0..=250 => false,
            251..=950 => true,
            951.. => false,
        }
    }

    fn is_left_on_line(&self, adc: &mut Adc) -> bool {
        match self.left_sensor.analog_read(adc) {
            0..=250 => false,
            251..=950 => true,
            951.. => false,
        }
    }

    fn car_off_line(&self, adc: &mut Adc) -> Direction {
        // TODO: THIS NEEDS TO DO LINE PATHFINDING
        Direction::None
    }
}