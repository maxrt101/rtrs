use super::script::Runtime;

pub struct Command {
    pub name: &'static str,
    pub help: &'static str,
    pub handler: fn(&mut Runtime, &[&str]) -> i8
}

#[macro_export]
macro_rules! command {
    ($name:expr, $help:expr, $handler:expr) => {
        $crate::shell::command::Command { name: $name, help: $help, handler: $handler }
    };
}
