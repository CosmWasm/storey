//! A collection of traits/types for accessing storage and managing storage namespaces.
//!
//! [`StorageBackend`] and [`StorageBackendMut`] are for accessing the fundamental binary
//! key-value storage. You only need to interact with them if you're integrating `storey` with
//! a new storage backend.
//!
//! [`Storage`] and [`StorageMut`] provide a common interface for any binary storage type,
//! including a storage backend or a storage branch (namespace). Similarly, [`RevIterableStorage`]
//! and [`IterableStorage`] represent binary storage types that provide iteration. These traits
//! are something you might be interested in if you're implementing a new container.
//!
//! [`StorageBranch`] is a storage namespace. It can be used to divide a backend's key namespace
//! into smaller namespaces. This is a fundamental building block for the hierarchy of storage
//! containers. You only need to be aware of it if you're implementing a new container.

mod branch;

pub use branch::StorageBranch;
pub use storey_storage::{
    IterableStorage, RevIterableStorage, Storage, StorageBackend, StorageBackendMut, StorageMut,
};
