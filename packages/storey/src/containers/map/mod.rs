pub mod key;
mod key_encoding;
pub mod set;

use key::DefaultKeySet;
pub use key::{Key, OwnedKey};
use key_encoding::KeyEncoding;
use key_encoding::KeyEncodingT;

use std::{borrow::Borrow, marker::PhantomData};

use crate::storage::{IterableStorage, StorageBranch};

use self::key::DynamicKey;
use self::key::FixedSizeKey;

use super::BoundFor;
use super::BoundedIterableAccessor;
use super::IterableAccessor;
use super::IterableStorable;
use super::NonTerminal;
use super::Storable;
use super::Terminal;

/// A map that stores values of type `V` under keys of type `K`.
///
/// The subkeys managed by the map are length-prefixed and appended to the map's prefix.
///
/// A map does not directly manage the storage of its values. Instead, it doles out access to
/// a collection of other containers.
///
/// # Key sets
///
/// The `KS` type parameter is the "key set" used. This is a marker type that
/// specifies the kind of keys that can be used with the map. The default key
/// set is [`DefaultKeySet`]. Providing another key set is an extension mechanism -
/// third party crates can define their own key set types to support third-party key types,
/// without bumping into orphan rules.
///
/// An example of a custom key set implementation is shown in the [`Key`] trait documentation.
///
/// # Examples
///
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::{Item, Map, router};
///
/// router! {
///     router Root {
///         0 -> item: Map<String, Item<u64, TestEncoding>>,
///     }
/// }
///
/// # let mut storage = TestStorage::new();
/// let mut root_access = Root::access(&mut storage);
/// let mut access = root_access.item_mut();
///
/// access.entry_mut("foo").set(&1337).unwrap();
/// assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
/// assert_eq!(access.entry("bar").get().unwrap(), None);
/// ```
///
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::{Item, Map, router};
///
/// router! {
///     router Root {
///         0 -> item: Map<String, Map<String, Item<u64, TestEncoding>>>,
///     }
/// }
///
/// # let mut storage = TestStorage::new();
/// let mut root_access = Root::access(&mut storage);
/// let mut access = root_access.item_mut();
///
/// access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
/// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), Some(1337));
/// assert_eq!(access.entry("foo").entry("baz").get().unwrap(), None);
/// ```
pub struct Map<K: ?Sized, V, KS = DefaultKeySet> {
    phantom: PhantomData<(*const K, V, KS)>,
}

impl<K, V, KS> Storable for Map<K, V, KS> {
    type Kind = NonTerminal;
    type Accessor<S> = MapAccess<K, V, S, KS>;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, S, KS> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
    }
}

impl<K, V, KS> IterableStorable for Map<K, V, KS>
where
    K: OwnedKey<KS>,
    V: IterableStorable,
    <V as IterableStorable>::KeyDecodeError: std::fmt::Display,
    (K::Kind, V::Kind): KeyEncodingT,
{
    type Key = (K, V::Key);
    type KeyDecodeError = MapKeyDecodeError<V::KeyDecodeError>;
    type Value = V::Value;
    type ValueDecodeError = V::ValueDecodeError;

    fn decode_key(key: &[u8]) -> Result<Self::Key, MapKeyDecodeError<V::KeyDecodeError>> {
        let behavior = <(K::Kind, V::Kind)>::BEHAVIOR;

        match behavior {
            KeyEncoding::LenPrefix => {
                let len = *key.first().ok_or(MapKeyDecodeError::EmptyKey)? as usize;

                if key.len() < len + 1 {
                    return Err(MapKeyDecodeError::KeyTooShort(len));
                }

                let map_key =
                    K::from_bytes(&key[1..len + 1]).map_err(|_| MapKeyDecodeError::InvalidUtf8)?;
                let rest = V::decode_key(&key[len + 1..]).map_err(MapKeyDecodeError::Inner)?;

                Ok((map_key, rest))
            }
            KeyEncoding::UseRest => {
                let map_key = K::from_bytes(key).map_err(|_| MapKeyDecodeError::InvalidUtf8)?;
                let rest = V::decode_key(&[]).map_err(MapKeyDecodeError::Inner)?;

                Ok((map_key, rest))
            }
            KeyEncoding::UseN(n) => {
                let map_key =
                    K::from_bytes(&key[..n]).map_err(|_| MapKeyDecodeError::InvalidUtf8)?;
                let rest = V::decode_key(&key[n..]).map_err(MapKeyDecodeError::Inner)?;

                Ok((map_key, rest))
            }
        }
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        V::decode_value(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid key length, expected empty key")]
pub enum MapKeyDecodeError<I: std::fmt::Display> {
    #[error("empty key, expected length prefix (1 byte)")]
    EmptyKey,

    #[error("key too short, expected {0} bytes after length prefix")]
    KeyTooShort(usize),

    #[error("invalid UTF8")]
    InvalidUtf8,

    #[error("sub key decode error: {0}")]
    Inner(I),
}

impl<I: std::fmt::Display> crate::error::StoreyError for MapKeyDecodeError<I> {}

/// An accessor for a map.
///
/// The accessor provides methods for interacting with the map in storage.
pub struct MapAccess<K: ?Sized, V, S, KS = DefaultKeySet> {
    storage: S,
    phantom: PhantomData<(*const K, V, KS)>,
}

impl<K, V, S, KS> MapAccess<K, V, S, KS>
where
    K: Key<KS>,
    V: Storable,
    (K::Kind, V::Kind): KeyEncodingT,
{
    /// Returns an immutable accessor for the inner container of this map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Map<String, Item<u64, TestEncoding>>,
    ///     }
    /// }
    ///
    /// # let mut storage = TestStorage::new();
    /// let mut root_access = Root::access(&mut storage);
    /// let mut access = root_access.item_mut();
    ///
    /// assert_eq!(access.entry("foo").get().unwrap(), None);
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Map<String, Map<String, Item<u64, TestEncoding>>>,
    ///     }
    /// }
    ///
    /// # let mut storage = TestStorage::new();
    /// let mut root_access = Root::access(&mut storage);
    /// let mut access = root_access.item_mut();
    ///
    /// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), None);
    /// ```
    pub fn entry<Q>(&self, key: &Q) -> V::Accessor<StorageBranch<&S>>
    where
        K: Borrow<Q>,
        Q: Key<KS, Kind = K::Kind> + ?Sized,
    {
        let behavior = <(K::Kind, V::Kind)>::BEHAVIOR;

        let key = match behavior {
            KeyEncoding::LenPrefix => len_prefix(key.encode()),
            _ => key.encode(),
        };

        V::access_impl(StorageBranch::new(&self.storage, key))
    }

    /// Returns a mutable accessor for the inner container of this map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Map<String, Item<u64, TestEncoding>>,
    ///     }
    /// }
    ///
    /// # let mut storage = TestStorage::new();
    /// let mut root_access = Root::access(&mut storage);
    /// let mut access = root_access.item_mut();
    ///
    /// access.entry_mut("foo").set(&1337).unwrap();
    /// assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Map<String, Map<String, Item<u64, TestEncoding>>>,
    ///     }
    /// }
    ///
    /// # let mut storage = TestStorage::new();
    /// let mut root_access = Root::access(&mut storage);
    /// let mut access = root_access.item_mut();
    ///
    /// access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
    /// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), Some(1337));
    /// ```
    pub fn entry_mut<Q>(&mut self, key: &Q) -> V::Accessor<StorageBranch<&mut S>>
    where
        K: Borrow<Q>,
        Q: Key<KS, Kind = K::Kind> + ?Sized,
    {
        let behavior = <(K::Kind, V::Kind)>::BEHAVIOR;

        let key = match behavior {
            KeyEncoding::LenPrefix => len_prefix(key.encode()),
            _ => key.encode(),
        };

        V::access_impl(StorageBranch::new(&mut self.storage, key))
    }
}

fn len_prefix<T: AsRef<[u8]>>(bytes: T) -> Vec<u8> {
    let len = bytes.as_ref().len();
    let mut result = Vec::with_capacity(len + 1);
    result.extend_from_slice(&(len as u8).to_be_bytes());
    result.extend_from_slice(bytes.as_ref());
    result
}

impl<K, V, S, KS> IterableAccessor for MapAccess<K, V, S, KS>
where
    K: OwnedKey<KS>,
    V: IterableStorable,
    <V as IterableStorable>::KeyDecodeError: std::fmt::Display,
    S: IterableStorage,
    (K::Kind, V::Kind): KeyEncodingT,
{
    type Storable = Map<K, V>;
    type Storage = S;

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }
}

// The following dance is necessary to make bounded iteration unavailable for maps
// that have both dynamic keys and "non-terminal" values (i.e. maps of maps, maps of columns, etc).
//
// This is because in cases where the key is dynamically size **and** there's another key
// after it, we have to length-prefix the key. This makes bounded iteration behave differently
// than in other cases (and rather unintuitively).

impl<K, V, S, KS> BoundedIterableAccessor for MapAccess<K, V, S, KS>
where
    K: OwnedKey<KS>,
    V: IterableStorable,
    <V as IterableStorable>::KeyDecodeError: std::fmt::Display,
    S: IterableStorage,
    (K::Kind, V::Kind): BoundedIterationAllowed + KeyEncodingT,
{
}

trait BoundedIterationAllowed {}

impl<const L: usize> BoundedIterationAllowed for (FixedSizeKey<L>, Terminal) {}
impl<const L: usize> BoundedIterationAllowed for (FixedSizeKey<L>, NonTerminal) {}
impl BoundedIterationAllowed for (DynamicKey, Terminal) {}

impl<K, V, Q, KS> BoundFor<Map<K, V, KS>> for &Q
where
    K: Borrow<Q> + OwnedKey<KS>,
    V: Storable,
    Q: Key<KS> + ?Sized,
    (K::Kind, V::Kind): KeyEncodingT,
{
    fn into_bytes(self) -> Vec<u8> {
        let behavior = <(K::Kind, V::Kind)>::BEHAVIOR;

        match behavior {
            KeyEncoding::LenPrefix => len_prefix(self.encode()),
            _ => self.encode(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use super::*;

    use crate::containers::test_utils::BranchContainer;
    use crate::containers::Item;

    use mocks::backend::TestStorage;
    use mocks::encoding::TestEncoding;
    use storey_storage::Storage as _;

    #[test]
    fn map() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();

        assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
        assert_eq!(
            storage.get(&[0, 102, 111, 111]),
            Some(1337u64.to_le_bytes().to_vec())
        );

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").remove();

        assert_eq!(access.entry("foo").get().unwrap(), None);
        assert_eq!(access.entry("bar").get().unwrap(), None);
    }

    #[test]
    fn bounded_iter_dyn_map_of_item() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();
        access.entry_mut("baz").set(&69).unwrap();

        let items = access
            .bounded_pairs(Bound::Included("bar"), Bound::Excluded("bazz"))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            items,
            vec![(("bar".to_string(), ()), 42), (("baz".to_string(), ()), 69)]
        );

        let items = access
            .bounded_pairs(Bound::Excluded("bar"), Bound::Included("foo"))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            items,
            vec![
                (("baz".to_string(), ()), 69),
                (("foo".to_string(), ()), 1337)
            ]
        );
    }

    #[test]
    fn iter_static_map_of_item() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();
        access.entry_mut("baz").set(&69).unwrap();

        let items = access.pairs().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(
            items,
            vec![
                (("bar".to_string(), ()), 42),
                (("baz".to_string(), ()), 69),
                (("foo".to_string(), ()), 1337)
            ]
        );
    }

    #[test]
    fn bounded_iter_static_map_of_map() {
        type MapOfMaps = BranchContainer<0, Map<u32, Map<String, Item<u64, TestEncoding>>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfMaps::access(&mut storage);

        access.entry_mut(&2).entry_mut("bar").set(&1337).unwrap();
        access.entry_mut(&3).entry_mut("baz").set(&42).unwrap();
        access.entry_mut(&4).entry_mut("quux").set(&69).unwrap();

        let items = access
            .bounded_pairs(Bound::Included(&2), Bound::Excluded(&4))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(
            items,
            vec![
                ((2, ("bar".to_string(), ())), 1337),
                ((3, ("baz".to_string(), ())), 42)
            ]
        );
    }

    #[test]
    fn pairs() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let items = access.pairs().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(
            items,
            vec![
                (("bar".to_string(), ()), 42),
                (("foo".to_string(), ()), 1337)
            ]
        );
    }

    #[test]
    fn keys() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let keys = access.keys().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(keys, vec![("bar".to_string(), ()), ("foo".to_string(), ())])
    }

    #[test]
    fn values() {
        type MapOfItems = BranchContainer<0, Map<String, Item<u64, TestEncoding>>>;

        let mut storage = TestStorage::new();

        let mut access = MapOfItems::access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let values = access.values().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(values, vec![42, 1337])
    }
}
