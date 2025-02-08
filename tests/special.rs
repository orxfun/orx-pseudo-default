use orx_pseudo_default::*;

fn take_pseudo_default<T: PseudoDefault>() {
    let _ = T::pseudo_default();
}

#[test]
fn special() {
    take_pseudo_default::<&str>();
}
