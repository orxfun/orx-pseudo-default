use crate::impl_on_generic_type;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

impl_on_generic_type!(VecDeque<T>, T, VecDeque::new());
impl_on_generic_type!(LinkedList<T>, T, LinkedList::new());
impl_on_generic_type!(HashSet<T>, T, HashSet::new());

impl_on_generic_type!(HashMap<K, V>, K, V, HashMap::new());
