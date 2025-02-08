use crate::impl_on_type;
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize,
};

impl_on_type!(AtomicBool, false.into());
impl_on_type!(AtomicI8, 0.into());
impl_on_type!(AtomicI16, 0.into());
impl_on_type!(AtomicI32, 0.into());
impl_on_type!(AtomicI64, 0.into());
impl_on_type!(AtomicIsize, 0.into());
impl_on_type!(AtomicU8, 0.into());
impl_on_type!(AtomicU16, 0.into());
impl_on_type!(AtomicU32, 0.into());
impl_on_type!(AtomicU64, 0.into());
impl_on_type!(AtomicUsize, 0.into());
