mod impls;
mod key_set;
mod kind;

pub use impls::{ArrayDecodeError, InvalidUtf8, NumericKeyDecodeError};
pub use key_set::DefaultKeySet;
pub use kind::{DynamicKey, FixedSizeKey, KeyKind};

pub use storey_macros::Key;
/// A key that can be used with a [`Map`](super::Map).
///
/// # Key sets
///
/// The `KS` type parameter is the "key set" used. This is a marker type that
/// specifies the kind of keys that can be used with the map. The default key
/// set is [`DefaultKeySet`]. Providing another key set is an extension mechanism -
/// third party crates can define their own key set types to support third-party key types,
/// getting around orphan rules.
///
/// # Examples
///
/// This example shows how to define an alternative key set type. To use it with a map,
/// the map also needs to be parameterized with the key set type;
///
/// ```
/// use storey::containers::map::{Key, OwnedKey};
/// use storey::containers::map::key::{DynamicKey, FixedSizeKey};
///
/// pub struct MyKeySet;
///
/// // imagine this is a third-party type
/// pub struct ExtType;
///
/// pub struct MyKey(String);
///
/// impl Key<MyKeySet> for MyKey {
///     type Kind = DynamicKey;
///
///     fn encode(&self) -> Vec<u8> {
///         self.0.as_bytes().to_vec()
///     }
/// }
///
/// impl OwnedKey<MyKeySet> for MyKey {
///     type Error = std::string::FromUtf8Error;
///
///     fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
///          String::from_utf8(bytes.to_vec()).map(|s| MyKey(s))
///     }
/// }
///
/// impl Key<MyKeySet> for ExtType {
///     type Kind = FixedSizeKey<16>;
///
///     fn encode(&self) -> Vec<u8> {
///         todo!()
///     }
/// }
///
/// impl OwnedKey<MyKeySet> for ExtType {
///     type Error = ();
///
///     fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
///          todo!()
///     }
/// }
///
/// // use the key set with a map
/// type MyMap = storey::containers::map::Map<MyKey, u32, MyKeySet>;
/// ```
pub trait Key<KS = DefaultKeySet> {
    /// The kind of key, meaning either fixed size or dynamic size.
    type Kind: KeyKind;

    /// Encode the key into a byte vector.
    fn encode(&self) -> Vec<u8>;
}

/// An owned key that can be used with a [`Map`](super::Map).
///
/// # Key sets
///
/// The `KS` type parameter is the "key set" used. This is a marker type that
/// specifies the kind of keys that can be used with the map. The default key
/// set is [`DefaultKeySet`]. Providing another key set is an extension mechanism -
/// third party crates can define their own key set types to support third-party key types,
/// without bumping into orphan rules.
///
/// An example of a custom key set is shown in the [`Key`] trait documentation.
pub trait OwnedKey<T = DefaultKeySet>: Key<T> {
    /// The error type that can occur when decoding the key.
    type Error;

    /// Decode the key from a byte slice.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_int_ordering() {
        let data = [-555555555, -3333, -1, 0, 1, 3333, 55555555];

        let mut encoded = data
            .iter()
            .map(|&x| Key::<DefaultKeySet>::encode(&x))
            .collect::<Vec<_>>();
        encoded.sort();

        let decoded = encoded
            .iter()
            .map(|x| <i32 as OwnedKey<DefaultKeySet>>::from_bytes(x).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(&data[..], &decoded);
    }

    #[test]
    fn signed_int_encoding() {
        // negative values have the leftmost bit unset
        assert_eq!(
            Key::<DefaultKeySet>::encode(&i32::MIN),
            [0b00000000, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&-2000i32),
            [0b01111111, 0xff, 248, 48]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&-3i32),
            [0b01111111, 0xff, 0xff, 0xfd]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&-2i32),
            [0b01111111, 0xff, 0xff, 0xfe]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&-1i32),
            [0b01111111, 0xff, 0xff, 0xff]
        );

        // non-negative values are BE encoded, but with the leftmost bit set
        assert_eq!(
            Key::<DefaultKeySet>::encode(&0i32),
            [0b10000000, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&1i32),
            [0b10000000, 0x00, 0x00, 0x01]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&2i32),
            [0b10000000, 0x00, 0x00, 0x02]
        );
        assert_eq!(
            Key::<DefaultKeySet>::encode(&i32::MAX),
            [0b11111111, 0xff, 0xff, 0xff]
        );
    }
}
