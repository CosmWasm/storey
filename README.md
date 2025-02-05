# storey

`storey` is an experimental storage abstraction layer for blockchain modules and smart contracts. It's mostly intended
for the [CosmWasm](https://cosmwasm.com/) ecosystem, where it's meant as an alternative to [`cw-storage-plus`](https://github.com/CosmWasm/cw-storage-plus).

In essence, if you have a binary key-value based store, you can integrate `storey` with it to provide typed abstractions that are
a little nicer to work with.

## Guide

A guide can be found [here](https://docs.cosmwasm.com/storey).

## Goals

- comparable user experience for simple use cases to [`cw-storage-plus`](https://github.com/CosmWasm/cw-storage-plus)
- minimal key length (compared to [`cw-storage-plus`](https://github.com/CosmWasm/cw-storage-plus))
- backend-agnostic core - go, plug it into something other than _CosmWasm_
- encoding-agnostic core - the _CosmWasm_-specific crate mandates an encoding for values, but the core itself doesn't
- minimal-dependency core - currently, the core has almost no dependencies
- composable collections
- a better API for folks who would like to implement their own collections

## Non-goals

It's unlikely we'll build these. It's more likely we'll eventually provide enough support that you can build them yourself without much headache, and nudge you in the right direction.

- indexes
- snapshots
