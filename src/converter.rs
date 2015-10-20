use num::pow;
use std::cmp;

// TODO use Bigint for 10^24
pub fn convert(number: f64) -> () {
  // TODO Use is_sign_positive
  // https://doc.rust-lang.org/std/primitive.f64.html?search=is_sign_positive
  // let negative = number < 0;

  // TODO check unit size
  let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  // TODO fix variable name later
  let x = 1000_f64;
  let a = (number.ln() / x.ln());
  let exponent = a.floor();

  // https://doc.rust-lang.org/std/primitive.f64.html#method.exp2
  // exp2 can be used

  let z = pow(1000, exponent as usize);

  // TODO return generated strings
  println!("{:?}", exponent);
  println!("{:?}", pow(1000, 3));
}
