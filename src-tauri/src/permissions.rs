#[cfg(windows)]
use is_elevated::is_elevated;

#[cfg(windows)]
pub fn elevated() -> bool {
    is_elevated()
}

#[cfg(not(windows))]
pub fn elevated() -> bool {
    true
}