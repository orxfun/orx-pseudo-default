use crate::impl_from_default;
use alloc::vec::Vec;

impl_from_default!(String);

impl_from_default!(T, Option<T>);
impl_from_default!(T, Vec<T>);
