//! Storage containers for use with [*CosmWasm*] smart contracts.
//!
//! [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm

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

pub use storey::containers::Map;
