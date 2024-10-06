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
        

        pub const $identifier: LazyLock<Identifier> = LazyLock::new(|| Identifier($string_i.to_string()));
    };
}