use cosmwasm_std::{Addr, Int128, Int256, Int512, Int64, Uint128, Uint256, Uint512, Uint64};
use storey::containers::map::key::{
    DynamicKey, FixedSizeKey, KeySetDefaults, NumericKeyDecodeError,
};
use storey::containers::map::{Key, OwnedKey};

/// The CosmWasm key set for use with storey's [`Map`](storey::containers::Map).
///
/// This key set includes the usual standard library types (like `u32` or `String`) as well as `cosmwasm_std` types (like `Addr` and `Uint128`).
///
/// For more information about key sets, take a look at the [`storey::containers::map::Key`] trait.
#[derive(KeySetDefaults)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_ints_1() {
        let test_vector = [
            (Uint64::from(0u64), [0, 0, 0, 0, 0, 0, 0, 0]),
            (Uint64::from(1u64), [0, 0, 0, 0, 0, 0, 0, 1]),
            (
                Uint64::from(0x1234567890abcdefu64),
                [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef],
            ),
        ];

        for (num, expected) in test_vector.iter() {
            let encoded = num.encode();
            assert_eq!(encoded, *expected);
        }

        for (expected, bytes) in test_vector.iter() {
            let decoded = Uint64::from_bytes(bytes).unwrap();
            assert_eq!(decoded, *expected);
        }
    }

    #[test]
    fn unsigned_ints_2() {
        let test_vector = [
            (Uint256::from(0u64), [0; 32]),
            (
                Uint256::new([
                    0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90,
                    0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34,
                    0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                ]),
                [
                    0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90,
                    0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 0x12, 0x34,
                    0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
                ],
            ),
        ];

        for (num, expected) in test_vector.iter() {
            let encoded = num.encode();
            assert_eq!(encoded, *expected);
        }

        for (expected, bytes) in test_vector.iter() {
            let decoded = Uint256::from_bytes(bytes).unwrap();
            assert_eq!(decoded, *expected);
        }
    }

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
