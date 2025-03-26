//! Implementations of the `Key`/`OwnedKey` trait for Rust std types.
//!
//! If adding a new implementation here, please also add a delegation
//! to the `key_set` module in `storey-macros`. This ensures consumers
//! of the `KeySetDefaults` derive macro can use the new types without
//! having to manually implement the `Key` trait themselves.

use super::{key_set::KeySet, DynamicKey, FixedSizeKey, Key, OwnedKey};

impl<KS: KeySet> Key<KS> for String {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<KS: KeySet> Key<KS> for Box<str> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<KS: KeySet> Key<KS> for str {
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

impl<KS: KeySet> OwnedKey<KS> for String {
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

impl<KS: KeySet> OwnedKey<KS> for Box<str> {
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

impl<KS: KeySet> Key<KS> for Vec<u8> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.clone()
    }
}

impl<KS: KeySet> Key<KS> for Box<[u8]> {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<KS: KeySet> Key<KS> for [u8] {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<KS: KeySet, const N: usize> Key<KS> for [u8; N] {
    type Kind = FixedSizeKey<N>;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<KS: KeySet> OwnedKey<KS> for Vec<u8> {
    type Error = ();

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(bytes.to_vec())
    }
}

impl<KS: KeySet> OwnedKey<KS> for Box<[u8]> {
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

impl<KS: KeySet, const N: usize> OwnedKey<KS> for [u8; N] {
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
            impl<KS: KeySet> Key<KS> for $t {
                type Kind = FixedSizeKey<{(Self::BITS / 8) as usize}>;

                fn encode(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }

            impl<KS: KeySet> OwnedKey<KS> for $t {
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
            impl<KS: KeySet> Key<KS> for $t {
                type Kind = FixedSizeKey<{(Self::BITS / 8) as usize}>;

                fn encode(&self) -> Vec<u8> {
                   (*self as $ut ^ <$t>::MIN as $ut).to_be_bytes().to_vec()
                }
            }

            impl<KS: KeySet> OwnedKey<KS> for $t {
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
