use stork::{DecodableWith as _, EncodableWith as _};

mod common;

#[test]
fn encoding() {
    let data = 12u64.encode().unwrap();
    assert_eq!(data, vec![12, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn decoding() {
    let data = vec![12, 0, 0, 0, 0, 0, 0, 0];
    let item = <u64>::decode(&data).unwrap();
    assert_eq!(item, 12);
}
