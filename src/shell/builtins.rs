use crate::shell::script::Runtime;
use super::LOGGER;

use core::fmt::Write;

pub fn cmd_help(rt: &mut Runtime, args: &[&str]) -> i8 {
    if let Some(name) = args.get(0) {
        for cmd in rt.commands {
            if cmd.name == *name {
                crate::info!("{} - {}", cmd.name, cmd.help);
                return 0;
            }
        }

        1
    } else {
        for cmd in rt.commands {
            crate::info!("{} - {}", cmd.name, cmd.help);
        }

        0
    }
}


pub fn cmd_echo(_rt: &mut Runtime, args: &[&str]) -> i8 {
    for arg in args {
        crate::print!("{} ", arg);
    }
    crate::println!();

    0
}


pub fn cmd_env(rt: &mut Runtime, args: &[&str]) -> i8 {
    if let Some(name) = args.get(0) {
        if let Some(val) = rt.env.get(name) {
            crate::info!("{}={}", name, val);
            return 0;
        }

        1
    } else {
        for key in rt.env.keys() {
            crate::info!("{}={}", key, rt.env.get(key).unwrap_or("<?>"));
        }

        0
    }
}

pub fn cmd_set(rt: &mut Runtime, args: &[&str]) -> i8 {
    if args.len() != 2 {
        crate::error!("Usage: set NAME VALUE");
        return 1;
    }

    let _ = rt.env.set(args[0], args[1]);

    0
}
