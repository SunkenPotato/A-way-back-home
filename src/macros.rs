
/// ## Emulate ternary operations in Rust 
/// Usage:
/// ```rs
/// 
/// let x = 5;
/// let condition = ternary!(x > 0; true, false);
/// assert_eq!(condition, true);
/// ```
#[macro_export]
macro_rules! ternary {
    ($cond:expr; $then:expr, $else:expr) => {
        if $cond {
            $then
        } else {
            $else
        }
    };
}