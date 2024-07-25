/// Implements PseudoDefault from Default.
#[macro_export]
macro_rules! impl_from_default {
    ($t:ty) => {
        impl crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                Default::default()
            }
        }
    };
    ($a:ident, $t:ty) => {
        impl<$a> crate::PseudoDefault for $t {
            fn pseudo_default() -> Self {
                Default::default()
            }
        }
    };
}
