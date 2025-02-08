use crate::impl_on_static_life_type;

impl_on_static_life_type!(usize, 0);
impl_on_static_life_type!(u8, 0);
impl_on_static_life_type!(u16, 0);
impl_on_static_life_type!(u32, 0);
impl_on_static_life_type!(u64, 0);
impl_on_static_life_type!(u128, 0);
impl_on_static_life_type!(isize, 0);
impl_on_static_life_type!(i8, 0);
impl_on_static_life_type!(i16, 0);
impl_on_static_life_type!(i32, 0);
impl_on_static_life_type!(i64, 0);
impl_on_static_life_type!(i128, 0);

impl_on_static_life_type!(char, '0');
impl_on_static_life_type!(bool, false);
impl_on_static_life_type!((), ());
