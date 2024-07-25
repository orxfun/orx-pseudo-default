# orx-pseudo-default

[![orx-pseudo-default crate](https://img.shields.io/crates/v/orx-pseudo-default.svg)](https://crates.io/crates/orx-pseudo-default)
[![orx-pseudo-default documentation](https://docs.rs/orx-pseudo-default/badge.svg)](https://docs.rs/orx-pseudo-default)

`PseudoDefault` ⊇ `Default` trait allows to create a cheap default instance of a type, which **does not claim to be useful**.

The difference of `PseudoDefault` from `Default` is the relaxed expectation of the created instance to be useful.

The main use case of the trait is when we need to create a cheap instance of a type without any arguments, only to throw away afterwards. Therefore, created instance does not need to be a decent one.

This trait allows to avoid unsafe code in certain use cases. For instance:
* We can avoid tricks such as uninit, manually-drop, etc. that requires to be extremely careful, when we could've actually created a valid instance much more easily.
* We can use pseudo-default to fill the gaps when we need to take out an element from a collection of types that cannot implement Default.

Note that pseudo-default requirement is more relaxed than that of default, and hence,
* types implementing Default is the subset of types implementing PseudoDefault,
* in other words, every type implementing Default automatically implements PseudoDefault,
* additionally, types that cannot implement Default can manually implement PseudoDefault, provided that it is safe and cheap to create a pseudo instance of the type without any arguments.

# Example

Consider the following fictional type `Share` which divides a whole into pieces. Without providing the `number_of_shares`, this type does not have a meaning.

**Therefore, we cannot justify implementing `Default`, it would be misleading.**

If we still need to be able to create Share's for some reason, we can simply use `pseudo_default`. We would know that the created type does not promise to make sense behaviorally; however, it is still a cheap and valid instance that can be safely dropped.

```rust
use orx_pseudo_default::PseudoDefault;

struct Share {
    number_of_shares: std::num::NonZeroUsize,
}

impl Share {
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
```

A more advanced use case could be the following. Assume that we are trying to create a vec wrapper called `TakeVec` with the following features;
* it allows to take out elements by index by a method called `take`
* we should be able to wrap an allocated vec without any additional allocation
* we need to be able to give back the originally allocated vec
* we want to achieve this without unsafe code

It is trivial to implement this with `Default` but we want to be less restrictive on the constraint so that it works for non-default types as well. We can use PseudoDefault for this.

```rust
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

// auto implemented default types

let mut vec: TakeVec<_> = vec![0, 1, 2, 3].into();
assert_eq!(vec.take(2), Some(2));

let mut vec: TakeVec<_> = vec![0.to_string(), 1.to_string()].into();
assert_eq!(vec.take(0), Some(String::from("0")));

// non-default types

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
```

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-pseudo-default/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
