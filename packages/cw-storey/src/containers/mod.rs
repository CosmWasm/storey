//! Storage containers for use with [*CosmWasm*] smart contracts.
//!
//! [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm

mod key_set;

/// The [`storey::containers::Item`] type with the default encoding for [*CosmWasm*] smart
/// contracts.
///
/// [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm
pub type Item<T> = storey::containers::Item<T, crate::encoding::CwEncoding>;

/// The [`storey::containers::Column`] type with the default encoding for [*CosmWasm*] smart
/// contracts.
///
/// [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm
pub type Column<T> = storey::containers::Column<T, crate::encoding::CwEncoding>;

pub type Map<K, V> = storey::containers::Map<K, V, key_set::CwKeySet>;

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::Addr;
    use mocks::backend::TestStorage;

    #[test]
    fn map_addr() {
        let map: Map<Addr, Item<u32>> = Map::new(0);
        let mut storage = TestStorage::new();

        let key = Addr::unchecked("addr1");

        map.access(&mut storage).entry_mut(&key).set(&42).unwrap();

        assert_eq!(map.access(&storage).entry(&key).get().unwrap(), Some(42));
    }
}
