use cosmwasm_std::Addr;
use storey::containers::map::{key::DynamicKey, Key, OwnedKey};

pub struct CwKeySet;

impl Key<CwKeySet> for Addr {
    type Kind = DynamicKey;

    fn encode(&self) -> Vec<u8> {
        self.as_str().as_bytes().to_vec()
    }
}

impl OwnedKey<CwKeySet> for Addr {
    type Error = <String as OwnedKey>::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        <String as OwnedKey>::from_bytes(bytes).map(Addr::unchecked)
    }
}

// delegate Key<CwKeySet> impls to their Key<DefaultKeySet> counterparts
macro_rules! key_delegate {
    ($($ty:ty),*) => {
        $(
            impl Key<CwKeySet> for $ty {
                type Kind = <$ty as Key>::Kind;

                fn encode(&self) -> Vec<u8> {
                    <Self as Key>::encode(self)
                }
            }
        )*
    }
}

key_delegate!(str, [u8]);

macro_rules! owned_key_delegate {
    ($($ty:ty),*) => {
        $(
            key_delegate!($ty);

            impl OwnedKey<CwKeySet> for $ty {
                type Error = <$ty as OwnedKey>::Error;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    <$ty as OwnedKey>::from_bytes(bytes)
                }
            }
        )*
    }
}

owned_key_delegate!(
    String,
    Box<str>,
    Vec<u8>,
    Box<[u8]>,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128
);

impl<const N: usize> Key<CwKeySet> for [u8; N] {
    type Kind = <Self as Key>::Kind;

    fn encode(&self) -> Vec<u8> {
        <Self as Key>::encode(self)
    }
}

impl<const N: usize> OwnedKey<CwKeySet> for [u8; N] {
    type Error = <Self as OwnedKey>::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        <Self as OwnedKey>::from_bytes(bytes)
    }
}
