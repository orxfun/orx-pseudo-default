#![cfg(feature = "std")]

use orx_pseudo_default::*;
use std::{rc::Rc, sync::Arc};

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn collections_std() {
    type T = String;

    take_pseudo_default::<Rc<T>>();
    take_pseudo_default::<Arc<T>>();
}
