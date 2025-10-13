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

#[cfg(feature = "shell_env_dynamic")]
pub type EnvVar = alloc::string::String;

#[cfg(feature = "shell_env_dynamic")]
pub type EnvStorage = alloc::collections::BTreeMap<EnvVar, alloc::string::String>;

#[cfg(feature = "shell_env_fixed")]
const SHELL_ENV_SIZE: usize = envint::envint!("RTRS_SHELL_ENV_SIZE", 16);

#[cfg(feature = "shell_env_fixed")]
const SHELL_VAR_SIZE: usize = envint::envint!("RTRS_SHELL_VAR_SIZE", 32);

#[cfg(feature = "shell_env_fixed")]
pub type EnvVar = heapless::String<SHELL_VAR_SIZE>;

#[cfg(feature = "shell_env_fixed")]
pub type EnvStorage = heapless::FnvIndexMap<EnvVar, EnvVar, SHELL_ENV_SIZE>;
