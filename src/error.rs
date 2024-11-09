use std::fmt::Display;

use bevy::log::warn_once;

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
            "WarnError:\n\tPossible bug: {}\n\tMessage: {}\n\tCode: {}",
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
    #[inline]
    pub fn trigger(&self) {
        self.trigger_msg("[]")
    }

    #[track_caller]
    pub fn trigger_msg<M>(&self, message: M)
    where
        M: Display,
    {
        warn_once!(
            "WarnError triggered from {}: {}. \nAdditional information: {}",
            core::panic::Location::caller(),
            self,
            message
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
    #[must_use = "Calling this may have an unexpected impact on the game loop"]
    pub fn exit_with_fatal_error(self) -> ! {
        panic!(
            "Fatal in-game error triggered from location: {}; {self}",
            core::panic::Location::caller()
        )
    }
}

pub mod errors {
    use super::FatalError;

    // START - FatalError
    pub const WINDOWS_NOT_INSTANTIATED: FatalError =
        FatalError::new("Windows have not yet been instantiated!", 0x001);
    // END - FatalError

    use super::WarnError;

    // START - WarnError
    pub const LEVEL_NOT_FOUND: WarnError =
        WarnError::new("The current level could not be obtained", 0x001, true);

    pub const SPAWNPOINT_ERR: WarnError =
        WarnError::new("Could not obtain spawnpoint for current level", 0x002, true);

    // END - WarnError
}
