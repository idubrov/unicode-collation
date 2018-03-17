[![crates.io](https://img.shields.io/crates/v/unicode-collate.svg)](https://crates.io/crates/unicode-collate)
[![crates.io](https://img.shields.io/crates/d/unicode-collate.svg)](https://crates.io/crates/unicode-collate)
[![CircleCI](https://img.shields.io/circleci/project/github/idubrov/unicode-collate.svg)](https://circleci.com/gh/idubrov/unicode-collate)

# unicode-collation

A [Unicode Collation Algorithm](https://www.unicode.org/reports/tr10/) implemented according
to Unicode Technical Standard #10.

## Usage

Add this to your *Cargo.toml*:
```toml
[dependencies]
unicode-collation = "0.1"
```

## Examples
Generate sort key for the given string:

```rust
extern crate unicode_collation;
use unicode_collation::{collate, CollationTable};

let table = CollationTable::from_text_file("data/allkeys.txt");
let key = collate("Hello!!!", &table);
assert_eq!(format!("{:?}", key), "[\
    1D7E 1D10 1DDD 1DDD 1E43 0261 0261 0261 | \
    0020 0020 0020 0020 0020 0020 0020 0020 | \
    0008 0002 0002 0002 0002 0002 0002 0002 |]");
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
