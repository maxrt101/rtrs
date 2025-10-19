use embedded_hal::digital::v2::OutputPin;

extern crate alloc;
use alloc::boxed::Box;

#[repr(transparent)]
pub struct Output {
    pin: Box<dyn OutputPin<Error = ()> + Send + Sync + 'static>,
}

impl Output {
    pub fn new<T: OutputPin<Error = ()> + Send + Sync + 'static>(pin: T) -> Self {
        Self { pin: Box::new(pin) }
    }

    pub fn set_high(&mut self) -> Result<(), ()> {
        self.pin.set_high()
    }

    pub fn set_low(&mut self) -> Result<(), ()> {
        self.pin.set_low()
    }
}

impl crate::object::Object for Output {}

#[macro_export]
macro_rules! output_pin_wrapper {
    ($name:ident, $type:ty) => {
        struct $name {
            pin: $type
        }
        
        impl $name {
            pub fn new(pin: $type) -> Self {
                Self { pin }
            }
        }
        
        impl embedded_hal::digital::v2::OutputPin for $name {
            type Error = ();
        
            fn set_low(&mut self) -> Result<(), Self::Error> {
                $crate::ok!(self.pin.set_low())
            }
        
            fn set_high(&mut self) -> Result<(), Self::Error> {
                $crate::ok!(self.pin.set_high())
            }
        }
    }
}
