use std::borrow::Borrow;

use storey_storage::{Storage, StorageMut};

use super::{key::DefaultKeySet, Key, NonTerminal, Storable};

/// A set of keys stored in the storage. This is effectively similar to
/// a `Map<K, ()>`, but more explicitly indicates that the keys are
/// the only thing that matters.
pub struct Set<K, KS = DefaultKeySet> {
    phantom: std::marker::PhantomData<(K, KS)>,
}

impl<K, KS> Storable for Set<K, KS> {
    type Kind = NonTerminal;
    type Accessor<S> = SetAccess<K, S, KS>;

    fn access_impl<S>(storage: S) -> SetAccess<K, S, KS> {
        SetAccess {
            storage,
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct SetAccess<K, S, KS = DefaultKeySet> {
    storage: S,
    phantom: std::marker::PhantomData<(K, KS)>,
}

impl<K, S, KS> SetAccess<K, S, KS>
where
    K: Key<KS>,
    S: Storage,
{
    /// Verify a key exists.
    pub fn has<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Key<KS, Kind = K::Kind> + ?Sized,
    {
        self.storage.has(&key.encode())
    }
}

impl<K, S, KS> SetAccess<K, S, KS>
where
    K: Key<KS>,
    S: StorageMut,
{
    /// Insert a key into the set.
    pub fn insert<Q>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Key<KS, Kind = K::Kind> + ?Sized,
    {
        self.storage.set(&key.encode(), &[])
    }

    /// Remove a key from the set.
    pub fn remove<Q>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Key<KS, Kind = K::Kind> + ?Sized,
    {
        self.storage.remove(&key.encode())
    }
}

#[cfg(test)]
mod tests {
    use mocks::backend::TestStorage;

    use super::*;

    use crate::containers::{test_utils::BranchContainer, Map};

    #[test]
    fn empty() {
        type MySet = BranchContainer<0, Set<String>>;

        let storage = TestStorage::new();

        assert!(!MySet::access(&storage).has("foo"));
    }

    #[test]
    fn set() {
        type MySet = BranchContainer<0, Set<String>>;

        let mut storage = TestStorage::new();

        MySet::access(&mut storage).insert("foo");

        assert!(MySet::access(&storage).has("foo"));
    }

    #[test]
    fn remove() {
        type MySet = BranchContainer<0, Set<String>>;

        let mut storage = TestStorage::new();

        MySet::access(&mut storage).insert("foo");
        assert!(MySet::access(&storage).has("foo"));

        MySet::access(&mut storage).remove("foo");
        assert!(!MySet::access(&storage).has("foo"));
    }

    #[test]
    fn map_of_set() {
        type MyMap = BranchContainer<0, Map<String, Set<String>>>;

        let mut storage = TestStorage::new();

        assert!(!MyMap::access(&storage).entry("foo").has("bar"));

        MyMap::access(&mut storage).entry_mut("foo").insert("bar");
        assert!(MyMap::access(&storage).entry("foo").has("bar"));
        assert!(!MyMap::access(&storage).entry("foob").has("ar"));

        MyMap::access(&mut storage).entry_mut("foob").insert("ar");
        assert!(MyMap::access(&storage).entry("foob").has("ar"));
    }
}
