#![cfg(feature = "std")]

use orx_pseudo_default::*;
use std::sync::atomic::*;

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn atomics() {
    take_pseudo_default::<AtomicBool>();
    take_pseudo_default::<AtomicI8>();
    take_pseudo_default::<AtomicI16>();
    take_pseudo_default::<AtomicI32>();
    take_pseudo_default::<AtomicI64>();
    take_pseudo_default::<AtomicIsize>();
    take_pseudo_default::<AtomicU8>();
    take_pseudo_default::<AtomicU16>();
    take_pseudo_default::<AtomicU32>();
    take_pseudo_default::<AtomicU64>();
    take_pseudo_default::<AtomicUsize>();
}
