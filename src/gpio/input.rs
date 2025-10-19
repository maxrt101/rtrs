use embedded_hal::digital::v2::InputPin;

extern crate alloc;
use alloc::boxed::Box;

#[repr(transparent)]
pub struct Input {
    pin: Box<dyn InputPin<Error = ()> + Send + Sync + 'static>,
}

impl Input {
    pub fn new<T: InputPin<Error = ()> + Send + Sync + 'static>(pin: T) -> Self {
        Self { pin: Box::new(pin) }
    }

    pub fn is_high(&self) -> Result<bool, ()> {
        self.pin.is_high()
    }

    pub fn is_low(&self) -> Result<bool, ()> {
        self.pin.is_low()
    }
}

impl crate::object::Object for Input {}

#[macro_export]
macro_rules! input_pin_wrapper {
    ($name:ident, $type:ty) => {
        struct $name {
            pin: $type
        }

        impl $name {
            pub fn new(pin: $type) -> Self {
                Self { pin }
            }
        }

        impl embedded_hal::digital::v2::InputPin for $name {
            type Error = ();

            fn is_high(&self) -> Result<bool, ()> {
                $crate::ok!(self.pin.is_high())
            }

            fn is_low(&self) -> Result<bool, ()> {
                $crate::ok!(self.pin.is_low())
            }
        }
    }
}
