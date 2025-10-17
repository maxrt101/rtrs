use embedded_hal::digital::v2::OutputPin;
use crate::{Action, Pattern, PatternExecutionContext};

extern crate alloc;
use alloc::boxed::Box;

pub struct Led<E> {
    pin: Box<dyn OutputPin<Error = E> + Send + Sync>,
    ctx: PatternExecutionContext
}

impl<E> Led<E> {
    pub fn new(pin: Box<dyn OutputPin<Error = E> + Send + Sync>) -> Self {
        Self {
            pin,
            ctx: PatternExecutionContext {
                pattern: None,
                index: 0,
                action_start: 0,
                action_duration: 0,
            }
        }
    }

    pub fn on(&mut self) {
        let _ = self.pin.set_high();
    }

    pub fn off(&mut self) {
        let _ = self.pin.set_low();
    }

    pub fn reset(&mut self) {
        self.ctx.reset();
        self.off();
    }

    pub fn set_pattern(&mut self, pattern: &'static Pattern) {
        self.reset();
        self.ctx.pattern = Some(pattern);
    }

    fn start_action(&mut self, ticks: u32) {
        self.ctx.action_start = crate::time::global_tick();
        self.ctx.action_duration = ticks;
    }

    pub fn is_running(&self) -> bool {
        self.ctx.pattern.is_some()
    }

    pub fn cycle(&mut self) {
        if let Some(pattern) = self.ctx.pattern {
            if crate::time::global_tick() != 0 && crate::time::global_tick() - self.ctx.action_start < self.ctx.action_duration {
                return;
            }

            if self.ctx.index >= pattern.actions.len() {
                self.reset();
                return;
            }

            match pattern.actions[self.ctx.index] {
                Action::On(ticks) => {
                    self.on();
                    self.start_action(ticks);
                }
                Action::Off(ticks) => {
                    self.off();
                    self.start_action(ticks);
                }
            }

            self.ctx.index += 1;
        }
    }
}

impl<E: 'static> crate::object::Object for Led<E> {}
