pub fn convert(num: f64) -> String {
  // TODO Use is_sign_positive
  // https://doc.rust-lang.org/std/primitive.f64.html?search=is_sign_positive
  // let negative = num < 0;

  // TODO check unit size
  let units = vec!["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  // TODO fix variable name later
  let delimiter = 1000_f64;
  let exponent = (num.ln() / delimiter.ln()).floor() as i32;
  let pretty_bytes = num / delimiter.powi(exponent);
  let unit = units[exponent as usize];
  format!("{} {}", pretty_bytes, unit)
}
