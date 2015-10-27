# rust-pretty-bytes [![Circle CI](https://img.shields.io/circleci/project/banyan/rust-pretty-bytes.svg)](https://circleci.com/gh/banyan/rust-pretty-bytes)

>Convert bytes to a human readable string: 1337 → 1.34 kB

Useful for displaying file sizes for humans, Ported from [sindresorhus/pretty-bytes](https://github.com/sindresorhus/pretty-bytes)

## Usage

### CLI

```shell
$ pretty-bytes 1337
1.34 kB

$ echo 1337 | pretty-bytes
1.34 kB
```

### API

```rust
extern crate pretty_bytes;
use pretty_bytes::converter::convert;

println!("{}", convert(1337_f64));
```

### License

MIT
