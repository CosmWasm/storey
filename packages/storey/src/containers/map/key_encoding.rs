use crate::containers::{NonTerminal, Terminal};

use super::key::{DynamicKey, FixedSizeKey};

/// A trait that specifies the encoding strategy for a key.
///
/// This trait is implemented on tuples of the form `(K, C)` where `K` is the key type (dynamic/fixed)
/// and `C` is the container type (terminal/nonterminal). Once we know these two properties, we can
/// determine the encoding strategy for the key.
///
/// Scenarios:
/// - If the key is dynamic and the container is nonterminal, then the key needs to be
///   length prefixed - otherwise, we would not know where the key ends and the key for the inner
///   container starts.
/// - If the key is dynamic and the container is terminal, then the key is the rest of the string.
/// - If the key is fixed size, then we statically provide the number of bytes to read/write.
pub trait KeyEncodingT {
    const BEHAVIOR: KeyEncoding;
}

impl KeyEncodingT for (DynamicKey, NonTerminal) {
    const BEHAVIOR: KeyEncoding = KeyEncoding::LenPrefix;
}

impl<const L: usize> KeyEncodingT for (FixedSizeKey<L>, Terminal) {
    const BEHAVIOR: KeyEncoding = KeyEncoding::UseRest;
}

impl KeyEncodingT for (DynamicKey, Terminal) {
    const BEHAVIOR: KeyEncoding = KeyEncoding::UseRest;
}

impl<const L: usize> KeyEncodingT for (FixedSizeKey<L>, NonTerminal) {
    const BEHAVIOR: KeyEncoding = KeyEncoding::UseN(L);
}

/// The encoding strategy for a given key.
pub enum KeyEncoding {
    /// The key needs to be length prefixed.
    LenPrefix,
    /// The key doesn't need to be length prefixed. The rest of the string is the key.
    UseRest,
    /// The key is of fixed size.
    UseN(usize),
}
