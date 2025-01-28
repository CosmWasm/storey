use cosmwasm_std::{Addr, Int128, Int256, Int512, Int64, Uint128, Uint256, Uint512, Uint64};
use storey::containers::map::key::{DynamicKey, FixedSizeKey, NumericKeyDecodeError};
use storey::containers::map::{Key, OwnedKey};

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

macro_rules! cosmwasm_std_uints1 {
    ($($ty:ty => $size:expr, $stdty:ty),*) => {
        $(
            impl Key<CwKeySet> for $ty {
                type Kind = FixedSizeKey<$size>;

                fn encode(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }

            impl OwnedKey<CwKeySet> for $ty {
                type Error = NumericKeyDecodeError;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    let array: [u8; $size] = bytes.try_into().map_err(|_| NumericKeyDecodeError::InvalidLength)?;
                    Ok(<$stdty>::from_be_bytes(array).into())
                }
            }
        )*
    }
}

cosmwasm_std_uints1!(
    Uint64 => 8, u64,
    Uint128 => 16, u128
);

macro_rules! cosmwasm_std_uints2 {
    ($($ty:ty => $size:expr),*) => {
        $(
            impl Key<CwKeySet> for $ty {
                type Kind = FixedSizeKey<$size>;

                fn encode(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }

            impl OwnedKey<CwKeySet> for $ty {
                type Error = NumericKeyDecodeError;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    let array: [u8; $size] = bytes.try_into().map_err(|_| NumericKeyDecodeError::InvalidLength)?;
                    Ok(<$ty>::from_be_bytes(array))
                }
            }
        )*
    }
}

cosmwasm_std_uints2!(
    Uint256 => 32,
    Uint512 => 64
);

macro_rules! cosmwasm_std_ints {
    ($($ty:ty => $size:expr),*) => {
        $(
            impl Key<CwKeySet> for $ty {
                type Kind = FixedSizeKey<$size>;

                fn encode(&self) -> Vec<u8> {
                    let mut bytes = self.to_be_bytes();
                    bytes[0] ^= 0x80;

                    bytes.to_vec()
                }
            }

            impl OwnedKey<CwKeySet> for $ty {
                type Error = NumericKeyDecodeError;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    let mut array: [u8; $size] = bytes.try_into().map_err(|_| NumericKeyDecodeError::InvalidLength)?;
                    array[0] ^= 0x80;

                    Ok(<$ty>::from_be_bytes(array))
                }
            }
        )*
    }
}

cosmwasm_std_ints!(Int64 => 8, Int128 => 16, Int256 => 32, Int512 => 64);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_ints() {
        let nums = [
            Int256::from(-542),
            Int256::from(-111),
            Int256::from(0),
            Int256::from(121),
            Int256::from(342),
        ];

        let mut byte_nums = nums.iter().map(|n| n.encode()).collect::<Vec<_>>();
        byte_nums.sort();

        let result = byte_nums
            .iter()
            .map(|bytes| Int256::from_bytes(bytes).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(result, nums);
    }
}
