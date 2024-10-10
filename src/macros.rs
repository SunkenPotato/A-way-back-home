#[macro_export]
macro_rules! ternary {
    ($cond:expr; $t:expr, $f:expr) => {
        if $cond {
            $t
        } else { $f }
    }
}

#[macro_export]
macro_rules! identifier {
    ($identifier:ident, $string_i:expr) => {
        pub const $identifier: std::sync::LazyLock<crate::components::component::Identifier> = std::sync::LazyLock::new(|| crate::components::component::Identifier($string_i.to_string()));
    };
}