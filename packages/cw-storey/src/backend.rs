use storey::storage::{IterableStorage, RevIterableStorage, StorageBackend, StorageBackendMut};

/// A wrapper around a type implementing [`cosmwasm_std::Storage`] that integrates it with [`storey`].
pub struct CwStorage<S>(pub S);

impl<S> StorageBackend for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(self.0, key)
    }
}

impl<S> StorageBackend for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(self.0, key)
    }
}

impl<S> StorageBackendMut for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn set(&mut self, key: &[u8], value: &[u8]) {
        cosmwasm_std::Storage::set(self.0, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        cosmwasm_std::Storage::remove(self.0, key)
    }
}

impl<S> IterableStorage for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Ascending)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Ascending)
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Ascending)
    }
}

impl<S> IterableStorage for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Ascending)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Ascending)
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Ascending)
    }
}

impl<S> RevIterableStorage for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Descending)
    }
}

impl<S> RevIterableStorage for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        self.0
            .range_keys(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        self.0
            .range_values(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        self.0.range(start, end, cosmwasm_std::Order::Descending)
    }
}
