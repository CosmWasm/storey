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
