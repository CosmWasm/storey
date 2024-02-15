use crate::{IterableStorage, RevIterableStorage, Storage, StorageMut};

pub struct StorageBranch<'s, S> {
    backend: &'s S,
    prefix: Vec<u8>,
}

impl<'s, S> StorageBranch<'s, S>
where
    S: Storage,
{
    pub fn new(backend: &'s S, prefix: Vec<u8>) -> Self {
        Self { backend, prefix }
    }
}

impl<S: Storage> Storage for StorageBranch<'_, S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.backend.get(&[&self.prefix[..], key].concat())
    }
}

impl<S: StorageMut> StorageMut for StorageBranch<'_, S> {
    fn set(&self, key: &[u8], value: &[u8]) {
        self.backend.set(&[&self.prefix[..], key].concat(), value)
    }

    fn remove(&self, key: &[u8]) {
        self.backend.remove(&[&self.prefix[..], key].concat())
    }
}

impl<S: IterableStorage> IterableStorage for StorageBranch<'_, S> {
    type KeysIterator<'a> = BranchKeysIter<S::KeysIterator<'a>> where Self: 'a;
    type ValuesIterator<'a> = S::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = BranchKVIter<S::PairsIterator<'a>> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        BranchKeysIter {
            inner: self.backend.keys(
                start.as_ref().map(AsRef::as_ref),
                end.as_ref().map(AsRef::as_ref),
            ),
            prefix_len: self.prefix.len(),
        }
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
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

impl<S: RevIterableStorage> RevIterableStorage for StorageBranch<'_, S> {
    type RevKeysIterator<'a> = BranchKeysIter<S::RevKeysIterator<'a>> where Self: 'a;
    type RevValuesIterator<'a> = S::RevValuesIterator<'a> where Self: 'a;
    type RevPairsIterator<'a> = BranchKVIter<S::RevPairsIterator<'a>> where Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
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
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.rev_values(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
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
    start: Option<&[u8]>,
    end: Option<&[u8]>,
) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
    if prefix.is_empty() {
        (start.map(|s| s.to_vec()), end.map(|s| s.to_vec()))
    } else {
        (
            Some(
                start
                    .map(|s| [prefix, s].concat())
                    .unwrap_or(prefix.to_vec()),
            ),
            Some(end.map(|e| [prefix, e].concat()).unwrap_or_else(|| {
                let mut pref = prefix.to_vec();
                pref.last_mut().map(|x| *x += 1);
                pref
            })),
        )
    }
}

pub struct BranchKeysIter<I> {
    inner: I,
    prefix_len: usize,
}

impl<'a, I> Iterator for BranchKeysIter<I>
where
    I: Iterator<Item = Vec<u8>>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|key| key[self.prefix_len..].to_vec())
    }
}

pub struct BranchKVIter<I> {
    inner: I,
    prefix_len: usize,
}

impl<'a, I> Iterator for BranchKVIter<I>
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

    // TODO: move TestStorage and use it for these unit tests?

    // use crate::backend::TestStorage;

    // #[test]
    // fn storage_branch() {
    //     let storage = TestStorage::new();
    //     let branch = StorageBranch::new(&storage, b"foo".to_vec());

    //     branch.set(b"bar", b"baz");
    //     branch.set(b"qux", b"quux");

    //     assert_eq!(storage.get(b"bar"), None);
    //     assert_eq!(storage.get(b"qux"), None);

    //     assert_eq!(storage.get(b"foobar"), Some(b"baz".to_vec()));
    //     assert_eq!(storage.get(b"fooqux"), Some(b"quux".to_vec()));
    // }

    #[test]
    fn sub_bounds_no_prefix() {
        assert_eq!(
            sub_bounds(&[], Some(b"foo"), Some(b"bar")),
            (Some(b"foo".to_vec()), Some(b"bar".to_vec()))
        );

        assert_eq!(
            sub_bounds(&[], Some(b"foo"), None),
            (Some(b"foo".to_vec()), None)
        );

        assert_eq!(
            sub_bounds(&[], None, Some(b"bar")),
            (None, Some(b"bar".to_vec()))
        );

        assert_eq!(sub_bounds(&[], None, None), (None, None));
    }

    #[test]
    fn sub_bounds_with_prefix() {
        assert_eq!(
            sub_bounds(b"foo", Some(b"bar"), Some(b"baz")),
            (Some(b"foobar".to_vec()), Some(b"foobaz".to_vec()))
        );

        assert_eq!(
            sub_bounds(b"foo", Some(b"bar"), None),
            (Some(b"foobar".to_vec()), Some(b"fop".to_vec()))
        );

        assert_eq!(
            sub_bounds(b"foo", None, Some(b"baz")),
            (Some(b"foo".to_vec()), Some(b"foobaz".to_vec()))
        );

        assert_eq!(
            sub_bounds(b"foo", None, None),
            (Some(b"foo".to_vec()), Some(b"fop".to_vec()))
        );
    }
}
