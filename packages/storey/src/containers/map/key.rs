/// A key that can be used with a [`Map`](super::Map).
pub trait Key {
    /// The kind of key, meaning either fixed size or dynamic size.
    type Kind: KeyKind;

    /// Encode the key into a byte vector.
    fn encode(&self) -> Vec<u8>;
}

/// An owned key that can be used with a [`Map`](super::Map).
pub trait OwnedKey: Key {
    /// The error type that can occur when decoding the key.
    type Error;

    /// Decode the key from a byte slice.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait IntoKey<O> {
    fn into_key(self) -> O;
}

impl<T> IntoKey<T> for (T,) {
    fn into_key(self) -> T {
        self.0
    }
}

pub trait IntoOwnedKey<O> {
    fn into_owned_key(self) -> O;
}

impl<T> IntoOwnedKey<T> for (T,) {
    fn into_owned_key(self) -> T {
        self.0
    }
}

impl Key for String {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Key for Box<str> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Key for str {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

/// An error type representing a failure to decode a UTF-8 string.
#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid UTF8")]
pub struct InvalidUtf8;

impl crate::error::StoreyError for InvalidUtf8 {}

impl OwnedKey for String {
    type Error = InvalidUtf8;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        std::str::from_utf8(bytes)
            .map(String::from)
            .map_err(|_| InvalidUtf8)
    }
}

impl OwnedKey for Box<str> {
    type Error = InvalidUtf8;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        std::str::from_utf8(bytes)
            .map(Box::from)
            .map_err(|_| InvalidUtf8)
    }
}

impl Key for Vec<u8> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Key for Box<[u8]> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl Key for [u8] {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<const N: usize> Key for [u8; N] {
    type Kind = FixedSizeKey<N>;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl OwnedKey for Vec<u8> {
    type Error = ();

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(bytes.to_vec())
    }
}

impl OwnedKey for Box<[u8]> {
    type Error = ();

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(bytes.to_vec().into_boxed_slice())
    }
}

/// An error type for decoding arrays.
#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
pub enum ArrayDecodeError {
    #[error("invalid length")]
    InvalidLength,
}

impl crate::error::StoreyError for ArrayDecodeError {}

impl<const N: usize> OwnedKey for [u8; N] {
    type Error = ArrayDecodeError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if bytes.len() != N {
            return Err(ArrayDecodeError::InvalidLength);
        }

        let mut buf = [0; N];
        buf.copy_from_slice(bytes);
        Ok(buf)
    }
}

/// A trait specifying the kind of key.
///
/// There are two kinds of keys: fixed-size keys and dynamic keys, which are
/// represented by the [`FixedSizeKey`] and [`DynamicKey`] types, respectively.
///
/// This trait is [sealed](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits)
/// and cannot be implemented outside of this crate.
pub trait KeyKind: sealed::KeyKindSeal {}

/// A marker type representing a fixed-size key.
pub struct FixedSizeKey<const L: usize>;

/// A marker type representing a dynamic-size key.
pub struct DynamicKey;

impl<const L: usize> KeyKind for FixedSizeKey<L> {}
impl KeyKind for DynamicKey {}

mod sealed {
    pub trait KeyKindSeal {}

    impl<const L: usize> KeyKindSeal for super::FixedSizeKey<L> {}
    impl KeyKindSeal for super::DynamicKey {}
}

/// An error type for decoding numeric keys.
#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
pub enum NumericKeyDecodeError {
    #[error("invalid length")]
    InvalidLength,
}

impl crate::error::StoreyError for NumericKeyDecodeError {}

macro_rules! impl_key_for_numeric {
    ($($t:ty),*) => {
        $(
            impl Key for $t {
                type Kind = FixedSizeKey<{(Self::BITS / 8) as usize}>;

                fn encode(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }

            impl OwnedKey for $t {
                type Error = NumericKeyDecodeError;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    if bytes.len() != std::mem::size_of::<Self>() {
                        return Err(NumericKeyDecodeError::InvalidLength);
                    }

                    let mut buf = [0; std::mem::size_of::<Self>()];
                    buf.copy_from_slice(bytes);
                    Ok(Self::from_be_bytes(buf))
                }
            }
        )*
    };
}

impl_key_for_numeric!(u8, u16, u32, u64, u128);

macro_rules! impl_key_for_signed {
    ($($t:ty : $ut:ty),*) => {
        $(
            impl Key for $t {
                type Kind = FixedSizeKey<{(Self::BITS / 8) as usize}>;

                fn encode(&self) -> Vec<u8> {
                   (*self as $ut ^ <$t>::MIN as $ut).to_be_bytes().to_vec()
                }
            }

            impl OwnedKey for $t {
                type Error = NumericKeyDecodeError;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    if bytes.len() != std::mem::size_of::<Self>() {
                        return Err(NumericKeyDecodeError::InvalidLength);
                    }

                    let mut buf = [0; std::mem::size_of::<Self>()];
                    buf.copy_from_slice(bytes);
                    Ok((Self::from_be_bytes(buf) as $ut ^ <$t>::MIN as $ut) as _)
                }
            }
        )*
    };
}

impl_key_for_signed!(i8 : u8, i16 : u16, i32 : u32, i64 : u64, i128 : u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_int_ordering() {
        let data = [-555555555, -3333, -1, 0, 1, 3333, 55555555];

        let mut encoded = data.iter().map(|&x| x.encode()).collect::<Vec<_>>();
        encoded.sort();

        let decoded = encoded
            .iter()
            .map(|x| i32::from_bytes(x).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(&data[..], &decoded);
    }

    #[test]
    fn signed_int_encoding() {
        // negative values have the leftmost bit unset
        assert_eq!((i32::MIN).encode(), [0b00000000, 0x00, 0x00, 0x00]);
        assert_eq!((-2000i32).encode(), [0b01111111, 0xff, 248, 48]);
        assert_eq!((-3i32).encode(), [0b01111111, 0xff, 0xff, 0xfd]);
        assert_eq!((-2i32).encode(), [0b01111111, 0xff, 0xff, 0xfe]);
        assert_eq!((-1i32).encode(), [0b01111111, 0xff, 0xff, 0xff]);

        // non-negative values are BE encoded, but with the leftmost bit set
        assert_eq!(0i32.encode(), [0b10000000, 0x00, 0x00, 0x00]);
        assert_eq!(1i32.encode(), [0b10000000, 0x00, 0x00, 0x01]);
        assert_eq!(2i32.encode(), [0b10000000, 0x00, 0x00, 0x02]);
        assert_eq!(i32::MAX.encode(), [0b11111111, 0xff, 0xff, 0xff]);
    }
}
