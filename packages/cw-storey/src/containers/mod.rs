//! Storage containers for use with [*CosmWasm*] smart contracts.
//!
//! [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm

mod key_set;

pub use key_set::CwKeySet;

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

/// The [`storey::containers::Map`] type with the [`CwKeySet`] key set, which includes the
/// usual standard library types (like `u32` or `String`) as well as `cosmwasm_std` types (like `Addr` and `Uint128`).
pub type Map<K, V> = storey::containers::Map<K, V, CwKeySet>;

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::Addr;
    use mocks::backend::TestStorage;
    use storey::containers::router;

    #[test]
    fn map_addr() {
        router! {
            router Root {
                0 -> map: Map<Addr, Item<u32>>,
            }
        }

        let mut storage = TestStorage::new();

        let key = Addr::unchecked("addr1");

        Root::access(&mut storage)
            .map_mut()
            .entry_mut(&key)
            .set(&42)
            .unwrap();

        assert_eq!(
            Root::access(&storage).map().entry(&key).get().unwrap(),
            Some(42)
        );
    }
}
