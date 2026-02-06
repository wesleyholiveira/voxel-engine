//! Facade public API crate

pub fn init() {
    println!("{} initialized", env!("CARGO_PKG_NAME"));
}
