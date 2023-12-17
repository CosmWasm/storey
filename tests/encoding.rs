mod common;

use stork::{DecodableWith as _, EncodableWith as _};

#[test]
fn encoding() {
    assert_eq!(12u64.encode(), Ok(12u64.to_le_bytes().to_vec()));
}

#[test]
fn decoding() {
    assert_eq!(<u64>::decode(&12u64.to_le_bytes()), Ok(12));
}
