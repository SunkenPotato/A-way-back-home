use std::fmt::Display;

use bevy::log::warn;

pub struct FatalError<T = &'static str>
where
    T: Display,
{
    pub message: T,
    pub exit_code: i32,
}

pub struct WarnError<T = &'static str>
where
    T: Display,
{
    pub message: T,
    pub possible_bug: bool,
    pub warn_code: i32,
}

impl<T> Display for FatalError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FatalError: Reason: {}; with exit code: {}",
            self.message, self.exit_code
        )
    }
}

impl<T> Display for WarnError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WarnError:\n\tPossible bug: {}\n\tMessage:{}\n\tCode:{}",
            self.possible_bug, self.message, self.warn_code
        )
    }
}

impl<T> WarnError<T>
where
    T: Display,
{
    pub const fn new(message: T, warn_code: i32, possible_bug: bool) -> Self {
        Self {
            message,
            warn_code,
            possible_bug,
        }
    }

    #[track_caller]
    pub fn trigger(&self) {
        warn!(
            "WarnError triggered from {}: {}",
            core::panic::Location::caller(),
            self
        )
    }
}

impl<T> FatalError<T>
where
    T: Display,
{
    pub const fn new(message: T, exit_code: i32) -> Self
    where
        T: Display,
    {
        Self { message, exit_code }
    }

    #[track_caller]
    #[allow(
        unsafe_code,
        reason = "Function is not unsafe, consequences might be bad."
    )]
    /// Not an unsafe method in itself, but the consequences might be.
    pub unsafe fn trigger(self) -> ! {
        bevy::log::error!(
            "Fatal in-game error triggered from location: {}; {self}",
            core::panic::Location::caller()
        );
        std::process::exit(self.exit_code)
    }
}

pub mod errors {
    // START - FatalError

    // END - FatalError

    use super::WarnError;

    // START - WarnError
    pub const LEVEL_NOT_FOUND: WarnError =
        WarnError::new("The current level could not be obtained", 0x001, true);

    // END - WarnError
}
