use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CustomSet<T> {
    set: BTreeSet<T>,
}

impl<T: Ord + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = BTreeSet::<T>::new();
        input.iter().for_each(|u| { set.insert(u.clone()); });
        CustomSet { set: set }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.set.is_disjoint(&other.set)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet { set: self.set.intersection(&other.set).cloned().collect() }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet { set: self.set.difference(&other.set).cloned().collect() }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        CustomSet { set: self.set.union(&other.set).cloned().collect() }
    }
}
