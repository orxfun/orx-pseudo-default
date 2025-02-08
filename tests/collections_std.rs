#![cfg(feature = "std")]

use orx_pseudo_default::*;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn collections_std() {
    type T = String;

    take_pseudo_default::<VecDeque<T>>();
    take_pseudo_default::<LinkedList<T>>();
    take_pseudo_default::<HashSet<T>>();
    take_pseudo_default::<HashMap<usize, T>>();
}
