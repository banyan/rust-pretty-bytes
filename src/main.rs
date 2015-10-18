extern crate pretty_bytes;

use std::env;
use pretty_bytes::cli;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    cli::run(env::args());
}
