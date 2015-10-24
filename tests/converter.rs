extern crate pretty_bytes;

use pretty_bytes::converter;

#[test]
fn it_works() {
    assert_eq!(converter::convert(1337_f64), "1.34 kB");
    assert_eq!(converter::convert(-1337_f64), "-1.34 kB");
}
