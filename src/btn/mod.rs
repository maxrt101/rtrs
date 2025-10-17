
use embedded_hal::digital::v2::InputPin;

extern crate alloc;

use alloc::boxed::Box;

pub struct Button<E> {
    pin: Box<dyn InputPin<Error = E> + Send + Sync + 'static>,
}

impl<E> Button<E> {
    pub fn new(pin: Box<dyn InputPin<Error = E> + Send + Sync + 'static>) -> Self {
        Self { pin }
    }

    pub fn is_high(&self) -> Result<bool, E> {
        self.pin.is_high()
    }

    pub fn is_low(&self) -> Result<bool, E> {
        self.pin.is_low()
    }
}

impl<E: 'static> crate::object::Object for Button<E> {}
