impl<B> super::StorageBackend for B
where
    B: cosmwasm_std::Storage,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        <B as cosmwasm_std::Storage>::get(self, key)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        <B as cosmwasm_std::Storage>::set(self, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        <B as cosmwasm_std::Storage>::remove(self, key)
    }
}

impl<B> super::StorageIterableBackend for B
where
    B: cosmwasm_std::Storage,
{
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        self.range(start, end, cosmwasm_std::Order::Ascending)
    }

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        self.range_keys(start, end, cosmwasm_std::Order::Ascending)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        self.range_values(start, end, cosmwasm_std::Order::Ascending)
    }
}

impl<B> super::StorageRevIterableBackend for B
where
    B: cosmwasm_std::Storage,
{
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        self.range(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        self.range_keys(start, end, cosmwasm_std::Order::Descending)
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        self.range_values(start, end, cosmwasm_std::Order::Descending)
    }
}
