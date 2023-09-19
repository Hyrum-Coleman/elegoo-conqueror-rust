use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::hal::port::{PB5, PB4};
use avr_device::atmega328p::TC1;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;

const TRIGGER_UP_TIME: u16 = 10u16;

pub struct SensorUnit {
    trig: Pin<Output, PB5>,
    echo: Pin<Input<Floating>, PB4>,
    timer: TC1,
}

impl SensorUnit {
    pub fn new(trig: Pin<Output, PB5>, echo: Pin<Input<Floating>, PB4>, timer: TC1) -> Self {
        Self {
            trig,
            echo,
            timer,
        }
    }

    pub fn return_distance(&mut self) -> u16 {
        let mut delay = arduino_hal::Delay::new();

        self.trig.set_high();
        delay.delay_us(TRIGGER_UP_TIME);
        self.trig.set_low();

        while self.echo.is_low() {

            if self.timer.tcnt1.read().bits() >= 65000 {
                return 63500;
            }
        }

        self.timer.tcnt1.write(|w| w.bits(0));

        while self.echo.is_high() {}

        (self.timer.tcnt1.read().bits()) / 58
    }
}