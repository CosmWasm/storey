pub struct MyKeySet;

#[test]
fn strings() {
    let data = ["foo", "", "🧗🏼‍♀️hi"];

    for s in data.iter() {
        let encoded = <str as storey::containers::map::Key>::encode(s);
        let my_encoded = <str as storey::containers::map::Key<MyKeySet>>::encode(s);
        assert_eq!(encoded, my_encoded);
    }

    for s in data.iter() {
        let s = s.to_string();

        let encoded = <String as storey::containers::map::Key>::encode(&s);
        let my_encoded = <String as storey::containers::map::Key<MyKeySet>>::encode(&s);
        assert_eq!(encoded, my_encoded);

        let my_decoded =
            <String as storey::containers::map::OwnedKey<MyKeySet>>::from_bytes(&encoded).unwrap();
        assert_eq!(s, my_decoded);
    }

    for s in data.iter() {
        let s: Box<str> = (*s).into();

        let encoded = <Box<str> as storey::containers::map::Key>::encode(&s);
        let my_encoded = <Box<str> as storey::containers::map::Key<MyKeySet>>::encode(&s);
        assert_eq!(encoded, my_encoded);

        let my_decoded =
            <Box<str> as storey::containers::map::OwnedKey<MyKeySet>>::from_bytes(&encoded)
                .unwrap();
        assert_eq!(s, my_decoded);
    }
}

#[test]
fn arrays() {
    let data = [5u8; 99999];

    let encoded = <[u8; 99999] as storey::containers::map::Key>::encode(&data);
    let my_encoded = <[u8; 99999] as storey::containers::map::Key<MyKeySet>>::encode(&data);

    assert_eq!(encoded, my_encoded);

    let my_decoded =
        <[u8; 99999] as storey::containers::map::OwnedKey<MyKeySet>>::from_bytes(&encoded).unwrap();
    assert_eq!(data, my_decoded);
}
