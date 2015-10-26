extern crate pretty_bytes;

use pretty_bytes::converter::convert;

#[test]
fn it_converts_bytes_to_human_readable_strings() {
    assert_eq!(convert(0_f64), "0 B");
    assert_eq!(convert(0.4_f64), "0.4 B");
    assert_eq!(convert(0.7_f64), "0.7 B");
    assert_eq!(convert(10_f64), "10 B");
    assert_eq!(convert(10.1_f64), "10.1 B");
    assert_eq!(convert(999_f64), "999 B");
    assert_eq!(convert(1001_f64), "1 kB");
    assert_eq!(convert(1e16), "10 PB");
    assert_eq!(convert(1e30), "1000000 YB");
}

#[test]
fn it_supports_negative_numbers() {
    assert_eq!(convert(-0.4_f64), "-0.4 B");
    assert_eq!(convert(-0.7_f64), "-0.7 B");
    assert_eq!(convert(-10.1_f64), "-10.1 B");
    assert_eq!(convert(-999_f64), "-999 B");
    assert_eq!(convert(-1001_f64), "-1 kB");
}
