use crate::impl_new_from_pseudo_default;
use core::{
    cell::{RefCell, UnsafeCell},
    mem::ManuallyDrop,
};

impl_new_from_pseudo_default!(Box<T>, T);
impl_new_from_pseudo_default!(UnsafeCell<T>, T);
impl_new_from_pseudo_default!(RefCell<T>, T);
impl_new_from_pseudo_default!(ManuallyDrop<T>, T);
