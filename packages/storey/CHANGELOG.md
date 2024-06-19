# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.2.1] - 2024-06-19


### Added
- add missing IterableStorage impls
## [0.2.0] - 2024-05-09


### Added
- *(storey)* return ix on `Column::push`
- provide typed bounds for iteration
- use single-byte keys for top-level containers

### Fixed
- *(storey)* Column::push bounds checking

### Other
- less awkward associated type names
- rename crates to packages

## [0.1.0](https://github.com/CosmWasm/storey/releases/tag/storey-v0.1.0) - 2024-03-22

### Other
- add Cargo manifest metadata for crates.io
- implement Cw backend + encoding
- rename crates to storey
