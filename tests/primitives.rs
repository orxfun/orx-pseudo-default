use orx_pseudo_default::*;

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn primitives() {
    take_pseudo_default::<usize>();
    take_pseudo_default::<u8>();
    take_pseudo_default::<u16>();
    take_pseudo_default::<u32>();
    take_pseudo_default::<u64>();
    take_pseudo_default::<u128>();
    take_pseudo_default::<isize>();
    take_pseudo_default::<i8>();
    take_pseudo_default::<i16>();
    take_pseudo_default::<i32>();
    take_pseudo_default::<i64>();
    take_pseudo_default::<i128>();
    take_pseudo_default::<char>();
    take_pseudo_default::<bool>();
    take_pseudo_default::<()>();
}

#[test]
fn primitives_ref() {
    take_pseudo_default::<&usize>();
    take_pseudo_default::<&u8>();
    take_pseudo_default::<&u16>();
    take_pseudo_default::<&u32>();
    take_pseudo_default::<&u64>();
    take_pseudo_default::<&u128>();
    take_pseudo_default::<&isize>();
    take_pseudo_default::<&i8>();
    take_pseudo_default::<&i16>();
    take_pseudo_default::<&i32>();
    take_pseudo_default::<&i64>();
    take_pseudo_default::<&i128>();
    take_pseudo_default::<&char>();
    take_pseudo_default::<&bool>();
    take_pseudo_default::<&()>();
}
