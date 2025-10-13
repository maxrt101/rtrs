
use std::collections::BTreeMap;

fn main() {
    let vars: BTreeMap<&str, &str> = [
        ("RTRS_STORAGE_FIXED_SIZE",  "16"),
        ("RTRS_LOG_META_FIXED_SIZE", "16"),
        ("RTRS_SHELL_INPUT_SIZE",    "32"),
        ("RTRS_SHELL_ARGS_SIZE",     "8"),
    ].iter().cloned().collect();
    
    for var in vars {
        let value = std::env::var(var.0).unwrap_or_else(|_| var.1.into());
        println!("cargo:rustc-env={}={}", var.0, value);
    }
}
