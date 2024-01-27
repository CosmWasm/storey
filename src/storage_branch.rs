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
    type KeysIterator<'a> = S::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = S::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = S::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.keys(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
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

        self.backend.pairs(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
    }
}

impl<S: RevIterableStorage> RevIterableStorage for StorageBranch<'_, S> {
    type RevKeysIterator<'a> = S::RevKeysIterator<'a> where Self: 'a;
    type RevValuesIterator<'a> = S::RevValuesIterator<'a> where Self: 'a;
    type RevPairsIterator<'a> = S::RevPairsIterator<'a> where Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        let (start, end) = sub_bounds(&self.prefix, start, end);

        self.backend.rev_keys(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
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

        self.backend.rev_pairs(
            start.as_ref().map(AsRef::as_ref),
            end.as_ref().map(AsRef::as_ref),
        )
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
