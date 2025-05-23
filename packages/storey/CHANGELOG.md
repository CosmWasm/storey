# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.5.0] - 2025-04-24


### Added
- add OwnedKey trait derive
- add Key trait derive

### Fixed
- [**breaking**] Remove unnecessary derive

### Tests
- test router! errors
## [0.4.0] - 2025-02-05


### Added
- add the `KeySetDefaults` derive
- make Map keys extendable
- improve access API to root containers
- add `Column::update`
- [**breaking**] add `update` method to `ItemAccess`
- inclusive/exclusive bounds
- bounded reverse iteration
- reverse iteration

### Fixed
- [**breaking**] rename `Column::update` to `Column::set`
- [**breaking**] make column id naming consistent
- [**breaking**] start column ids with 1
- [**breaking**] #63 sane ordering for signed map keys

## [0.3.0] - 2024-09-11

### Added

- implement {ItemAccess, ColumnAccess}::get_or
- implement {ItemAccess, ColumnAccess}::try_get
- add Key/OwnedKey impls for more std types
- add remove to ItemAccess
- add remove entry from map

### Fixed

- fix docs inner links
- fix visibility

### Documentation

- document `BoundedIterableAccessor`
- document map key types/trait/assoc types

## [0.2.1] - 2024-06-19

### Added

- add missing IterableStorage impls

## [0.2.0] - 2024-05-09

### Added

- _(storey)_ return ix on `Column::push`
- provide typed bounds for iteration
- use single-byte keys for top-level containers

### Fixed

- _(storey)_ Column::push bounds checking

### Other

- less awkward associated type names
- rename crates to packages

## [0.1.0](https://github.com/CosmWasm/storey/releases/tag/storey-v0.1.0) - 2024-03-22

### Other

- add Cargo manifest metadata for crates.io
- implement Cw backend + encoding
- rename crates to storey
