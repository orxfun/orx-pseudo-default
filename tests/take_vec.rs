use orx_pseudo_default::PseudoDefault;

struct TakeVec<T>(Vec<T>);

impl<T> From<Vec<T>> for TakeVec<T> {
    fn from(inner: Vec<T>) -> Self {
        Self(inner)
    }
}

impl<T> From<TakeVec<T>> for Vec<T> {
    fn from(value: TakeVec<T>) -> Self {
        value.0
    }
}

impl<T: PseudoDefault> TakeVec<T> {
    fn take(&mut self, index: usize) -> Option<T> {
        self.0.get_mut(index).map(|element| {
            let mut value = T::pseudo_default();
            std::mem::swap(&mut value, element);
            value
        })
    }
}

#[test]
fn take_vec_copy() {
    let mut vec: TakeVec<_> = vec![0, 1, 2, 3].into();
    assert_eq!(vec.take(2), Some(2));
    assert_eq!(vec.take(42), None);
}

#[test]
fn take_vec_non_copy() {
    let mut vec: TakeVec<_> = vec![0.to_string(), 1.to_string()].into();
    assert_eq!(vec.take(0), Some(String::from("0")));
    assert_eq!(vec.take(2), None);
}

#[test]
fn take_vec_non_default() {
    struct Share {
        number_of_shares: std::num::NonZeroUsize,
    }

    impl Share {
        #[allow(dead_code)]
        fn share_size(&self, whole_amount: usize) -> usize {
            whole_amount / self.number_of_shares
        }
    }

    impl PseudoDefault for Share {
        fn pseudo_default() -> Self {
            Self {
                number_of_shares: std::num::NonZeroUsize::new(1).unwrap(),
            }
        }
    }

    let mut vec: TakeVec<_> = vec![
        Share {
            number_of_shares: std::num::NonZeroUsize::new(42).unwrap(),
        },
        Share {
            number_of_shares: std::num::NonZeroUsize::new(7).unwrap(),
        },
    ]
    .into();
    assert_eq!(vec.take(0).map(|x| x.number_of_shares.into()), Some(42));
    assert!(vec.take(2).is_none());
}
