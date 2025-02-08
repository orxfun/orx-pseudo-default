use crate::impl_new_from_pseudo_default;
use std::{rc::Rc, sync::Arc};

impl_new_from_pseudo_default!(Rc<T>, T);

impl_new_from_pseudo_default!(Arc<T>, T);
