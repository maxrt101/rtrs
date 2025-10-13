extern crate alloc;

#[cfg(feature = "shell_input_dynamic")]
pub type Input = alloc::string::String;

#[cfg(feature = "shell_input_fixed")]
const SHELL_INPUT_SIZE: usize = envint::envint!("RTRS_SHELL_INPUT_SIZE", 32);

#[cfg(feature = "shell_input_fixed")]
pub type Input = heapless::String<SHELL_INPUT_SIZE>;

#[cfg(feature = "shell_args_dynamic")]
pub type Arguments<'a> = alloc::vec::Vec<&'a str>;

#[cfg(feature = "shell_args_fixed")]
const SHELL_ARGS_SIZE: usize = envint::envint!("RTRS_SHELL_ARGS_SIZE", 8);

#[cfg(feature = "shell_args_fixed")]
pub type Arguments<'a> = heapless::Vec<&'a str, SHELL_ARGS_SIZE>;
