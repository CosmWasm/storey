# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.4.1] - 2025-02-05


### Added
- add the `KeySetDefaults` derive
- make Map keys extendable
- improve access API to root containers
- easy conversion to cosmwasm_std::StdError

### Fixed
- use msgpack impl provided by cosmwasm_std

### Tests
- have cw_storey smoke tests use mock cw contract

## [0.4.0] - 2024-09-11

### Added

- add remove to ItemAccess
- add remove entry from map

### Documentation

- fix indentation

## [0.3.1] - 2024-06-19

### Fixed

- make CW integration work with &mut dyn Storage

## [0.3.0] - 2024-06-12

### Added

- use msgpack encoding provided by cosmwasm-std

## [0.2.0] - 2024-05-09

### Added

- use single-byte keys for top-level containers

### Other

- rename crates to packages

## [0.1.0](https://github.com/CosmWasm/storey/releases/tag/cw-storey-v0.1.0) - 2024-03-22

### Added

- _(cw-storey)_ re-export storey containers
- _(cw-storey)_ replace CBOR with MessagePack

### Other

- add Cargo manifest metadata for crates.io
- _(cw-storey)_ add a simple smoke test
- implement Cw backend + encoding
- rename crates to storey
