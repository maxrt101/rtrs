use embedded_hal::digital::v2::OutputPin;

extern crate alloc;
use alloc::boxed::Box;

pub struct Output<E> {
    pin: Box<dyn OutputPin<Error = E> + Send + Sync + 'static>,
}

impl<E> Output<E> {
    pub fn new(pin: Box<dyn OutputPin<Error = E> + Send + Sync + 'static>) -> Self {
        Self { pin }
    }

    pub fn set_high(&mut self) -> Result<(), E> {
        self.pin.set_high()
    }

    pub fn set_low(&mut self) -> Result<(), E> {
        self.pin.set_low()
    }
}

impl<E: 'static> crate::object::Object for Output<E> {}
