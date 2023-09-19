use arduino_hal::Adc;
use crate::hardware::motor::Direction;
use super::echo_module::SensorUnit;
use super::irmodule::IrModule;

pub struct CombinedSensor {
    ir_sensor: IrModule,
    distance_sensor: SensorUnit,
}

impl CombinedSensor {

    pub fn new(ir_sensor: IrModule, distance_sensor: SensorUnit) -> Self {
        Self {
            ir_sensor,
            distance_sensor,
        }
    }

    pub fn get_direction(&mut self, adc: &mut Adc) -> Direction {

        match self.distance_sensor.return_distance() {
            0..=2 => return self.avoid_object(),
            _ => {()}
        }


       self.ir_sensor.get_direction(adc)
    }

    fn avoid_object(&mut self) -> Direction {
        // TODO: OBJECT AVOIDANCE LOGIC
        Direction::None
    }
}