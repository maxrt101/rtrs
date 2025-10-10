use crate::Action;

pub struct Pattern {
    pub actions: &'static [Action],
}

pub struct PatternExecutionContext {
    pub pattern: Option<&'static Pattern>,
    pub index: usize,
    pub action_start: u32,
    pub action_duration: u32,
}

impl PatternExecutionContext {
    pub fn reset(&mut self) {
        self.pattern = None;
        self.index = 0;
        self.action_start = 0;
        self.action_duration = 0;
    }
}

#[macro_export]
macro_rules! led_pattern {
    ( $( $action:expr ), * $(,)? ) => {
        rtrs::led::Pattern {
            actions: &[
                $($action),*
            ],
        }
    };
}
