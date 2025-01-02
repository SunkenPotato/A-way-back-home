#[macro_export]
macro_rules! impl_intcell {
    ($t:ty | $id:expr; ($l:expr, $h:expr)) => {
        impl $crate::world::IntCell for $t {
            const DIMENSIONS: (f32, f32) = ($l, $h);
            const INTCELL_ID: i32 = $id;
        }
    };

    ($t:ty | $id:expr; $l:expr) => {
        impl_intcell!($t | $id; ($l, $l));
    }
}
