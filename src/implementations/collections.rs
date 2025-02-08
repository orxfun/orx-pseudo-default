extern crate alloc;

use crate::{impl_on_generic_type, impl_on_static_life_generic_type, impl_on_type};
use alloc::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

impl_on_type!(String, String::new());

impl_on_generic_type!(Vec<T>, T, Vec::new());
impl_on_generic_type!(BTreeSet<T>, T, BTreeSet::new());

impl_on_generic_type!(BTreeMap<K, V>, K, V, BTreeMap::new());

impl_on_static_life_generic_type!(Option<T>, T, None);
