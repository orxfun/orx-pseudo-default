use core::{
    cell::{RefCell, UnsafeCell},
    mem::ManuallyDrop,
};
use orx_pseudo_default::*;

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn collections_std() {
    type T = String;

    take_pseudo_default::<Box<T>>();
    take_pseudo_default::<UnsafeCell<T>>();
    take_pseudo_default::<RefCell<T>>();
    take_pseudo_default::<ManuallyDrop<T>>();
}
