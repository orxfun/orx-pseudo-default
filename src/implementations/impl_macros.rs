/// Implements PseudoDefault on type T.
#[macro_export]
macro_rules! impl_on_type {
    ($t:ty, $value:expr) => {
        impl $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                $value
            }
        }
    };
}

/// Implements PseudoDefault on type T and &'a T.
#[macro_export]
macro_rules! impl_on_static_life_type {
    ($t:ty, $value:expr) => {
        impl $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                $value
            }
        }

        impl<'a> $crate::PseudoDefault for &'a $t {
            fn pseudo_default() -> Self {
                &$value
            }
        }
    };
}

/// Implements PseudoDefault on generic type `G<T>` or `G<T, U>.`
#[macro_export]
macro_rules! impl_on_generic_type {
    ($t:ty, $g:ident, $value:expr) => {
        impl<$g> $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                $value
            }
        }
    };

    ($t:ty, $g:ident, $h:ident, $value:expr) => {
        impl<$g, $h> $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                $value
            }
        }
    };
}

/// Implements PseudoDefault on generic type `G<T>` and `&'a G<T>`
#[macro_export]
macro_rules! impl_on_static_life_generic_type {
    ($t:ty, $g:ident, $value:expr) => {
        impl<$g> $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                $value
            }
        }

        impl<'a, $g> $crate::PseudoDefault for &'a $t {
            fn pseudo_default() -> Self {
                &$value
            }
        }
    };
}

/// Implements PseudoDefault on `G<T>` by calling `G::new(T::pseudo_default())`
#[macro_export]
macro_rules! impl_new_from_pseudo_default {
    ($t:ty, $g:ident) => {
        impl<$g: $crate::PseudoDefault> $crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                Self::new($g::pseudo_default())
            }
        }
    };
}
