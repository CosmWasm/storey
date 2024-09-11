# cw-storey

`cw-storey` is a crate providing storage abstractions for _CosmWasm_ smart contracts. It's an alternative to [`cw-storage-plus`](https://crates.io/crates/cw-storage-plus).

The core logic lives in the [`storey`](https://crates.io/crates/storey) crate, while the `cw-storey` crate is an integration of `storey` with the `CosmWasm` storage interface. It also provides the recommended encoding for KV store values: [MessagePack](https://msgpack.org/index.html).

A (possibly still incomplete) guide can be found [here](https://docs.cosmwasm.com/storey).
