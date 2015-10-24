pub fn convert(num: f64) -> String {
  let negative = if num.is_sign_positive() { "" } else { "-" };
  let num = num.abs();

  // TODO check unit size
  let units = vec!["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  let delimiter = 1000_f64;
  let exponent = (num.ln() / delimiter.ln()).floor() as i32;
  let pretty_bytes = num / delimiter.powi(exponent);
  let unit = units[exponent as usize];
  format!("{}{:.2} {}", negative, pretty_bytes, unit)
}
