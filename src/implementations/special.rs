use crate::PseudoDefault;

impl PseudoDefault for &str {
    fn pseudo_default() -> Self {
        ""
    }
}
