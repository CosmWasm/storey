pub trait StorageBackend {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn remove(&mut self, key: &[u8]);

    fn has(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }
}

pub trait StorageIterableBackend: StorageBackend {
    type KeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type ValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type PairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    fn keys<'a>(&'a self, start: Option<&'a [u8]>, end: Option<&'a [u8]>)
        -> Self::KeysIterator<'a>;
    fn values<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::ValuesIterator<'a>;
    fn pairs<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::PairsIterator<'a>;
}

pub trait StorageRevIterableBackend: StorageBackend {
    type RevKeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type RevValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;
    type RevPairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevKeysIterator<'a>;
    fn rev_values<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevValuesIterator<'a>;
    fn rev_pairs<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevPairsIterator<'a>;
}
