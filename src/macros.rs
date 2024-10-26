use bevy::log::warn_once;
use std::fmt::Display;

#[macro_export]
macro_rules! ternary {
    ($cond:expr; $t:expr, $f:expr) => {
        if $cond {
            $t
        } else {
            $f
        }
    };
}

#[macro_export]
macro_rules! identifier {
    ($identifier:ident, $string_i:expr) => {
        pub static $identifier: std::sync::LazyLock<$crate::components::component::Identifier> =
            std::sync::LazyLock::new(|| {
                $crate::components::component::Identifier($string_i.to_string())
            });
    };
}

// Not exactly a macro, but it fits best here.
#[track_caller]
pub fn warn_fn<T>(msg: Option<T>)
where
    T: Display,
{
    let message = format!(
        "Function emitted a warning at: {}; Reason: {}",
        core::panic::Location::caller(),
        msg.map_or("Unknown".to_string(), |v| v.to_string())
    );

    warn_once!(message);
}

#[macro_export]
macro_rules! warn_fn {
    () => {

        let pos = format!("{}:{}:{}", file!(), line!(), column!());
        let message = format!("Function emitted a warning at: {pos}! Reason: Unknown.")

        bevy::log::warn_once!(message);
    };

    ($msg:expr) => {
        let pos = format!("{}:{}:{}", file!(), line!(), column!());

        let message = format!("Function emitted a warning at: {pos}! Reason: {}", $msg);

        bevy::log::warn_once!(message);
    }
}
