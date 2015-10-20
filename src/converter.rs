use std::cmp;

pub fn convert(num: f64) -> () {
  println!("{:?}", num);

  // TODO Use is_sign_positive
  // https://doc.rust-lang.org/std/primitive.f64.html?search=is_sign_positive
  // let negative = num < 0;

  // TODO check unit size
  let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  // TODO fix variable name later
  let x = 1000_f64;
  let a = (num.ln() / x.ln());
  let exponent = a.floor();

  // https://doc.rust-lang.org/std/primitive.f64.html#method.exp2
  // exp2 can be used
  // num = (num / x.powf(exponent)) * 1;

  // TODO return generated strings
  println!("{:?}", num);
}
