#[cfg(target_os = "windows")]
#[path = "win/mod.rs"]
mod rsys;

pub use self::rsys::*;