use std::ops::Bound;

use crate::storage::{IterableStorage, RevIterableStorage, Storage, StorageMut};

/// A type representing a storage namespace created by applying a prefix to all keys.
///
/// This type implements the [`Storage`] and [`StorageMut`] traits, making the fact a prefix
/// is applied transparent to the user.
///
/// You don't need to be aware of this type unless implementing a custom container.
///
/// # Example
/// ```
/// # use mocks::backend::TestStorage;
/// use storey::storage::{Storage as _, StorageMut as _, StorageBranch};
///
/// let mut storage = TestStorage::new();
/// let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());
///
/// branch.set(b"bar", b"baz");
///
/// assert_eq!(branch.get(b"bar"), Some(b"baz".to_vec()));
/// assert_eq!(storage.get(b"foobar"), Some(b"baz".to_vec()));
/// ```
pub struct StorageBranch<S> {
    backend: S,
    prefix: Vec<u8>,
}

impl<S> StorageBranch<S> {
    /// Creates a new `StorageBranch` instance given a prefix.
    pub fn new(backend: S, prefix: Vec<u8>) -> Self {
        Self { backend, prefix }
    }
}

impl<S: Storage> Storage for StorageBranch<&S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get(&[&self.prefix[..], key].concat())
    }

    fn get_meta(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get_meta(&[&self.prefix[..], key].concat())
    }
}

impl<S: Storage> Storage for StorageBranch<&mut S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get(&[&self.prefix[..], key].concat())
    }

    fn get_meta(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get_meta(&[&self.prefix[..], key].concat())
    }
}

impl<S: StorageMut> StorageMut for StorageBranch<&mut S> {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.backend.set(&[&self.prefix[..], key].concat(), value)
    }

    fn remove(&mut self, key: &[u8]) {
        self.backend.remove(&[&self.prefix[..], key].concat())
    }

    fn set_meta(&mut self, key: &[u8], value: &[u8]) {
        self.backend
            .set_meta(&[&self.prefix[..], key].concat(), value)
    }

    fn remove_meta(&mut self, key: &[u8]) {
        self.backend.remove_meta(&[&self.prefix[..], key].concat())
    }
}

impl<S: IterableStorage> IterableStorage for StorageBranch<&S> {
    type KeysIterator<'a> = BranchKeysIter<S::KeysIterator<'a>> where Self: 'a;
    type ValuesIterator<'a> = S::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = BranchKVIter<S::PairsIterator<'a>> where Self: 'a;

    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKeysIter {
            inner: self.backend.keys(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }

    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKVIter {
            inner: self.backend.pairs(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }
}

impl<S: IterableStorage> IterableStorage for StorageBranch<&mut S> {
    type KeysIterator<'a> = BranchKeysIter<S::KeysIterator<'a>> where Self: 'a;
    type ValuesIterator<'a> = S::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = BranchKVIter<S::PairsIterator<'a>> where Self: 'a;

    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKeysIter {
            inner: self.backend.keys(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }

    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKVIter {
            inner: self.backend.pairs(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }
}

impl<S: RevIterableStorage> RevIterableStorage for StorageBranch<&S> {
    type RevKeysIterator<'a> = BranchKeysIter<S::RevKeysIterator<'a>> where Self: 'a;
    type RevValuesIterator<'a> = S::RevValuesIterator<'a> where Self: 'a;
    type RevPairsIterator<'a> = BranchKVIter<S::RevPairsIterator<'a>> where Self: 'a;

    fn rev_keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::RevKeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKeysIter {
            inner: self.backend.rev_keys(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }

    fn rev_values<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.rev_values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKVIter {
            inner: self.backend.rev_pairs(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }
}

impl<S: RevIterableStorage> RevIterableStorage for StorageBranch<&mut S> {
    type RevKeysIterator<'a> = BranchKeysIter<S::RevKeysIterator<'a>> where Self: 'a;
    type RevValuesIterator<'a> = S::RevValuesIterator<'a> where Self: 'a;
    type RevPairsIterator<'a> = BranchKVIter<S::RevPairsIterator<'a>> where Self: 'a;

    fn rev_keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::RevKeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKeysIter {
            inner: self.backend.rev_keys(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }

    fn rev_values<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.rev_values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKVIter {
            inner: self.backend.rev_pairs(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }
}

fn sub_bounds(
    prefix: &[u8],
    start: Bound<&[u8]>,
    end: Bound<&[u8]>,
) -> (Bound<Vec<u8>>, Bound<Vec<u8>>) {
    if prefix.is_empty() {
        (start.map(|s| s.to_vec()), end.map(|s| s.to_vec()))
    } else {
        (
            // concat prefix and start if bounded
            // return just the prefix if unbounded
            if let Bound::Unbounded = start {
                Bound::Included(prefix.to_vec())
            } else {
                start.map(|s| [prefix, s].concat())
            },
            if let Bound::Unbounded = end {
                Bound::Excluded({
                    let mut pref = prefix.to_vec();
                    if let Some(x) = pref.last_mut() {
                        *x += 1;
                    }
                    pref
                })
            } else {
                end.map(|e| [prefix, e].concat())
            },
        )
    }
}

/// An iterator over the keys of a `StorageBranch`.
pub struct BranchKeysIter<I> {
    inner: I,
    prefix_len: usize,
}

impl<I> Iterator for BranchKeysIter<I>
where
    I: Iterator<Item = Vec<u8>>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|key| key[self.prefix_len..].to_vec())
    }
}

/// An iterator over the key-value pairs of a `StorageBranch`.
pub struct BranchKVIter<I> {
    inner: I,
    prefix_len: usize,
}

impl<I> Iterator for BranchKVIter<I>
where
    I: Iterator<Item = (Vec<u8>, Vec<u8>)>,
{
    type Item = (Vec<u8>, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, value)| {
            let key = key[self.prefix_len..].to_vec();
            (key, value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mocks::backend::TestStorage;

    #[test]
    fn storage_branch() {
        let mut storage = TestStorage::new();
        let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());

        branch.set(b"bar", b"baz");
        branch.set(b"qux", b"quux");

        assert_eq!(storage.get(b"bar"), None);
        assert_eq!(storage.get(b"qux"), None);

        assert_eq!(storage.get(b"foobar"), Some(b"baz".to_vec()));
        assert_eq!(storage.get(b"fooqux"), Some(b"quux".to_vec()));
    }

    #[test]
    fn sub_bounds_no_prefix() {
        assert_eq!(
            sub_bounds(&[], Bound::Included(b"foo"), Bound::Excluded(b"bar")),
            (
                Bound::Included(b"foo".to_vec()),
                Bound::Excluded(b"bar".to_vec())
            )
        );

        assert_eq!(
            sub_bounds(&[], Bound::Included(b"foo"), Bound::Unbounded),
            (Bound::Included(b"foo".to_vec()), Bound::Unbounded)
        );

        assert_eq!(
            sub_bounds(&[], Bound::Unbounded, Bound::Excluded(b"bar")),
            (Bound::Unbounded, Bound::Excluded(b"bar".to_vec()))
        );

        assert_eq!(
            sub_bounds(&[], Bound::Unbounded, Bound::Unbounded),
            (Bound::Unbounded, Bound::Unbounded)
        );
    }

    #[test]
    fn sub_bounds_with_prefix() {
        assert_eq!(
            sub_bounds(b"foo", Bound::Included(b"bar"), Bound::Excluded(b"baz")),
            (
                Bound::Included(b"foobar".to_vec()),
                Bound::Excluded(b"foobaz".to_vec())
            )
        );

        assert_eq!(
            sub_bounds(b"foo", Bound::Included(b"bar"), Bound::Unbounded),
            (
                Bound::Included(b"foobar".to_vec()),
                Bound::Excluded(b"fop".to_vec())
            )
        );

        assert_eq!(
            sub_bounds(b"foo", Bound::Unbounded, Bound::Excluded(b"baz")),
            (
                Bound::Included(b"foo".to_vec()),
                Bound::Excluded(b"foobaz".to_vec())
            )
        );

        assert_eq!(
            sub_bounds(b"foo", Bound::Unbounded, Bound::Unbounded),
            (
                Bound::Included(b"foo".to_vec()),
                Bound::Excluded(b"fop".to_vec())
            )
        );
    }

    #[test]
    fn pairs() {
        let mut storage = TestStorage::new();
        let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());

        branch.set(b"bar", b"baz");
        branch.set(b"qux", b"quux");

        let mut iter = branch.pairs(Bound::Unbounded, Bound::Unbounded);
        assert_eq!(iter.next(), Some((b"bar".to_vec(), b"baz".to_vec())));
        assert_eq!(iter.next(), Some((b"qux".to_vec(), b"quux".to_vec())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn keys() {
        let mut storage = TestStorage::new();
        let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());

        branch.set(b"bar", b"baz");
        branch.set(b"qux", b"quux");

        let mut iter = branch.keys(Bound::Unbounded, Bound::Unbounded);
        assert_eq!(iter.next(), Some(b"bar".to_vec()));
        assert_eq!(iter.next(), Some(b"qux".to_vec()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn values() {
        let mut storage = TestStorage::new();
        let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());

        branch.set(b"bar", b"baz");
        branch.set(b"qux", b"quux");

        let mut iter = branch.values(Bound::Unbounded, Bound::Unbounded);
        assert_eq!(iter.next(), Some(b"baz".to_vec()));
        assert_eq!(iter.next(), Some(b"quux".to_vec()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn meta() {
        let mut storage = TestStorage::new();
        let mut branch = StorageBranch::new(&mut storage, b"foo".to_vec());

        branch.set_meta(b"bar", b"baz");
        branch.set_meta(b"qux", b"quux");

        assert_eq!(storage.get_meta(b"bar"), None);
        assert_eq!(storage.get_meta(b"qux"), None);

        assert_eq!(storage.get_meta(b"foobar"), Some(b"baz".to_vec()));
        assert_eq!(storage.get_meta(b"fooqux"), Some(b"quux".to_vec()));

        assert_eq!(storage.get(b"foobar"), None);
        assert_eq!(storage.get(b"fooqux"), None);
    }
}
