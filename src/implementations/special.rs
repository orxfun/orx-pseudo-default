use crate::PseudoDefault;

impl<'a> PseudoDefault for &'a str {
    fn pseudo_default() -> Self {
        ""
    }
}
