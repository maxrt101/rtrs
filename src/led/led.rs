use embedded_hal::digital::v2::OutputPin;
use crate::{Action, Pattern, PatternExecutionContext};
use void::Void;

pub struct Led {
    // led: &'static str,
    pin: &'static mut (dyn OutputPin<Error = Void> + Send + Sync),
    ctx: PatternExecutionContext
}

impl Led {
    pub fn new(pin: &'static mut (dyn OutputPin<Error = Void> + Send + Sync)) -> Self {
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
        self.pin.set_high().unwrap();
    }

    pub fn off(&mut self) {
        self.pin.set_low().unwrap()
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
                // println!("WAIT action");
                return;
            }

            if self.ctx.index >= pattern.actions.len() {
                // println!("END pattern i={} t={} a.s={} a.d={}", self.ctx.index, self.ctx.tick, self.ctx.action_start, self.ctx.action_duration);
                self.reset();
                return;
            }

            match pattern.actions[self.ctx.index] {
                Action::On(ticks) => {
                    // println!("action {} ON t={} a.s={} a.d={}", self.ctx.index, self.ctx.tick, self.ctx.action_start, self.ctx.action_duration);
                    self.on();
                    self.start_action(ticks);
                }
                Action::Off(ticks) => {
                    // println!("action {} OFF t={} a.s={} a.d={}", self.ctx.index, self.ctx.tick, self.ctx.action_start, self.ctx.action_duration);
                    self.off();
                    self.start_action(ticks);
                }
            }

            self.ctx.index += 1;
        }
    }

    // pub fn tick(&mut self) {
    //     self.ctx.tick += 1;
    // }
}

impl crate::object::Object for Led {}
