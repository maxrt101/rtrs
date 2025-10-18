use crate::time::TickProvider;

extern crate alloc;
use alloc::boxed::Box;

#[derive(Copy, Clone)]
pub enum Command {
    On(u32),
    Off(u32),
    Goto(u32),
}

#[derive(Copy, Clone)]
pub enum Action {
    Command(Command),
    Repeat(Command, u32),
    // Repeat(Action, u32),
}

/*
Command(On(200)),
Command(Off(320)),
Repeat(Goto(0), 5)


*/

enum ActionResultAction {
    Idle,
    Increment,
    ProcessAgain,
}

pub struct Pattern {
    pub actions: &'static [Action],
}

pub struct PatternExecutionContext {
    pub pattern: Option<&'static Pattern>,
    pub index: usize,
    pub action_start: u32,
    pub action_duration: u32,
    pub repeat_counter: u32,
    pub tick: Box<dyn TickProvider<Tick = u32>>,
}

impl PatternExecutionContext {
    pub fn new(
        pattern: &'static Pattern,
        tick: Box<dyn TickProvider<Tick = u32>>
    ) -> Self {
        Self {
            pattern: Some(pattern),
            index: 0,
            action_start: 0,
            action_duration: 0,
            repeat_counter: 0,
            tick,
        }
    }

    pub fn reset(&mut self) {
        self.pattern         = None;
        self.index           = 0;
        self.action_start    = 0;
        self.action_duration = 0;
        self.repeat_counter  = 0;
    }

    pub fn start(&mut self, pattern: &'static Pattern) {
        self.reset();
        self.pattern = Some(pattern);
    }

    fn start_action(&mut self, ticks: u32) {
        self.action_start    = self.tick.get_tick();
        self.action_duration = ticks;
    }

    pub fn is_running(&self) -> bool {
        self.pattern.is_some()
    }

    fn process_action<E>(&mut self, action: &Action, pin: &mut super::Output<E>) -> ActionResultAction {
        match action {
            Action::Command(Command::On(ticks)) => {
                // println!("on {}", *ticks);
                let _ = pin.set_high();
                self.start_action(*ticks);
            }
            Action::Command(Command::Off(ticks)) => {
                // println!("off {}", *ticks);
                let _ = pin.set_low();
                self.start_action(*ticks);
            }
            Action::Command(Command::Goto(idx)) => {
                // println!("goto {}", *idx);
                self.index = *idx as usize;
                self.action_duration = 0;
                return ActionResultAction::ProcessAgain;
            }
            Action::Repeat(nested_action, times) => {
                // println!("repeat {}/{}", self.repeat_counter, *times);
                if self.repeat_counter == *times {
                    // self.index = current_idx;
                    self.repeat_counter = 0;
                    return ActionResultAction::Increment;
                }

                self.process_action(&Action::Command(*nested_action), pin);

                self.repeat_counter += 1;

                return ActionResultAction::Idle;
            }
        }

        ActionResultAction::Increment
    }

    fn process_current_action<E>(&mut self, pattern: &Pattern, pin: &mut super::Output<E>) {
        // print!("{} ", self.index);
        match self.process_action(&pattern.actions[self.index], pin) {
            ActionResultAction::Idle => {}
            ActionResultAction::Increment => {
                self.index += 1;
            }
            ActionResultAction::ProcessAgain => {
                self.process_current_action(pattern, pin);
            }
        }
    }

    pub fn cycle<E>(&mut self, pin: &mut super::Output<E>) {
        if let Some(pattern) = self.pattern {
            if self.tick.get_tick() != 0 && self.tick.get_tick() - self.action_start < self.action_duration {
                return;
            }

            if self.index >= pattern.actions.len() {
                self.reset();
                return;
            }

            self.process_current_action(pattern, pin);
        }
    }
}

#[macro_export]
macro_rules! gpio_pattern {
    ( $( $action:expr ), * $(,)? ) => {
        rtrs::gpio::Pattern {
            actions: &[
                $($action),*
            ],
        }
    };
}


