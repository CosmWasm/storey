pub trait Key {
    type Kind: KeyKind;

    fn encode(&self) -> Vec<u8>;
}

pub trait OwnedKey: Key {
    type Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Key for String {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid UTF8")]
pub struct InvalidUtf8;

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

pub trait KeyKind {}

pub struct FixedSizeKey<const L: usize>;
pub struct DynamicKey;

impl<const L: usize> KeyKind for FixedSizeKey<L> {}
impl KeyKind for DynamicKey {}

pub enum NumericKeyDecodeError {
    InvalidLength,
}

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

impl_key_for_numeric!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl<const N: usize> Key for [u8; N] {
    type Kind = FixedSizeKey<N>;

    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }
}
