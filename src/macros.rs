#[macro_export]
macro_rules! ternary {
    ($cond:expr; $t:expr, $f:expr) => {
        if $cond {
            $t
        } else { $f }
    }
}

#[macro_export]
macro_rules! optional_code {
    ($env_name:expr; $($code:block)*) => {
        if let Some(_v) = option_env!($env_name) {
            $(
                $code
            )*
        } else { () }
    };
}

#[macro_export]
macro_rules! identifier {
    ($i:expr) => {
        Identifier($i)
    };
}