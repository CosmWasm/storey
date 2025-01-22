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

/// A trait for converting a type into one that implements [`Storage`].
///
/// This trait is meant to be implemented for a tuple of the intended type, as in
/// `impl IntoStorage<T> for (T,)`. This is to allow blanket implementations on foreign
/// types without stumbling into [E0210](https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210).
///  
/// Implementing this trait for foreign types allows to use those foreign types directly
/// with functions like [`Item::access`](crate::containers::Item::access).
///
/// # Example
///
/// This example should give you an idea of how to allow a foreign type to be used directly with functions
/// like [`Item::access`](crate::containers::Item::access).
///
/// Blanket implementations should also be possible!
///
/// ```
/// use storey::storage::{IntoStorage, StorageBackend};
///
/// mod foreign_crate {
///     // This is a foreign type that we want to use with `storey`. Note that due to orphan rules,
///     // we can't implement `StorageBackend` for this type.
///     pub struct ExtStorage;
///
///     impl ExtStorage {
///         pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
///             todo!()
///         }
///
///         pub fn has(&self, key: &[u8]) -> bool {
///             todo!()
///         }
///     }
/// }
///
/// use foreign_crate::ExtStorage;
///
/// // Our wrapper can be used as a storage backend. It delegates all calls to the foreign type.
/// struct MyStorage(ExtStorage);
///
/// impl StorageBackend for MyStorage {
///     fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
///         self.0.get(key)
///     }
///
///     fn has(&self, key: &[u8]) -> bool {
///         self.0.has(key)
///     }
/// }
///
/// // Implementing `IntoStorage` like this makes it possible to use `ExtStorage` directly with
/// // functions like `Item::access`, without users having to wrap it in `MyStorage`.
/// impl IntoStorage<MyStorage> for (ExtStorage,) {
///     fn into_storage(self) -> MyStorage {
///         MyStorage(self.0)
///     }
/// }
/// ```
pub use storey_storage::IntoStorage;
