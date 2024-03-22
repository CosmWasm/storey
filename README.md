# _storey_

_storey_ is currently a pile of debris with some (hopefully) good ideas strewn about. One day, a new storage abstraction library for the decentralized world might rise from it. Like a phoenix from ashes. Or like a sleep-deprived commuter at 6 a.m. One of the two.

## Loose goals (subject to change)

- comparable user experience for simple use cases to [`cw-storage-plus`](https://github.com/CosmWasm/cw-storage-plus)
- minimal key length (compared to [`cw-storage-plus`](https://github.com/CosmWasm/cw-storage-plus))
- backend-agnostic core - go, plug it into something other than _CosmWasm_
- encoding-agnostic core - a _CosmWasm_-specific crate will mandate an encoding for values, but the core itself probably won't
- minimal-dependency core - currently, the core has 0 (zero) external dependencies
- composable collections
- nicer compound keys
- a better API for folks who would like to implement their own collections
- spit-shined code

## Non-goals

It's unlikely we'll build these. It's more likely we'll eventually provide enough support that you can build them yourself without much headache.

- indexes
- snapshots
