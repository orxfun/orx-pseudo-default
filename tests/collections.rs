extern crate alloc;

use alloc::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;
use orx_pseudo_default::*;

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn collections() {
    type T = String;

    take_pseudo_default::<String>();

    take_pseudo_default::<Vec<T>>();
    take_pseudo_default::<BTreeSet<T>>();

    take_pseudo_default::<BTreeMap<usize, T>>();

    take_pseudo_default::<Option<T>>();
}
