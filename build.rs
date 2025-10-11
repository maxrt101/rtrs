
fn main() {
    let storage_size = std::env::var("RTRS_STORAGE_FIXED_SIZE").unwrap_or_else(|_| "16".into());
    println!("cargo:rustc-env=RTRS_STORAGE_FIXED_SIZE={}", storage_size);

    let log_meta_size = std::env::var("RTRS_LOG_META_FIXED_SIZE").unwrap_or_else(|_| "16".into());
    println!("cargo:rustc-env=RTRS_LOG_META_FIXED_SIZE={}", log_meta_size);
}
