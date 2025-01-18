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

#[macro_export]
macro_rules! impl_entity {
    ($t:ty | $id:expr) => {
        impl crate::world::Entity for $t {
            const IDENTIFIER: &str = $id;
        }
    };

    ($t:ty | $id:expr; ($l:expr; $h:expr)) => {
        impl crate::world::Entity for $t {
            const IDENTIFIER: &str = $id;
            const DIMENSIONS: Option<(f32, f32)> = Some(($l, $h));
        }
    };

    ($t:ty | $id:expr; $l:expr) => {
        impl_entity!($t | $id; ($l; $l));
    };
}

#[macro_export]
macro_rules! query_as_single {
    ($field:ident; $query:expr) => {
        let Ok($field) = $query.get_single() else {
            return;
        };
    };

    (($($field:ident),*); $query:expr) => {
        let Ok(($($field),*)) = $query.get_single() else {
            return;
        };
    };

    (mut $field:ident; $query:expr) => {
        let Ok(mut $field) = $query.get_single_mut() else {
            return;
        }
    };

    (($($(&$m:tt)? $field:ident),*); $query:expr) => {
        let Ok(($($($m)? $field),*)) = $query.get_single_mut() else { return };
    };
}

#[macro_export]
macro_rules! sealed_trait {
    (
        $v:vis trait $name:ident $body:tt => impls $($sealed_type:ty)*
    ) => {
        ::paste::paste! {
            trait [<$name Sealed>] {}
            $(
                impl [<$name Sealed>] for $sealed_type {}
            )*
            #[allow(private_bounds)]
            $v trait $name: [<$name Sealed>] $body
        }
    };
}
