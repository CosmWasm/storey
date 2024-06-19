# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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
- *(cw-storey)* re-export storey containers
- *(cw-storey)* replace CBOR with MessagePack

### Other
- add Cargo manifest metadata for crates.io
- *(cw-storey)* add a simple smoke test
- implement Cw backend + encoding
- rename crates to storey
