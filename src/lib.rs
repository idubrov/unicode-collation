//! A [Unicode Collation Algorithm](https://www.unicode.org/reports/tr10/) implemented according
//! to Unicode Technical Standard #10.
//!
//! # Usage
//!
//! Add this to your *Cargo.toml*:
//! ```toml
//! [dependencies]
//! unicode-collation = "0.1"
//! ```
//!
//! # Examples
//! Generate sort key for the given string:
//!
//! ```rust
//! extern crate unicode_collation;
//! use unicode_collation::{collate, CollationTable};
//! 
//! # pub fn main() {
//! let table = CollationTable::from_text_file("data/allkeys.txt");
//! let key = collate("Hello!!!", &table);
//! assert_eq!(format!("{:?}", key), "[\
//!     1D7E 1D10 1DDD 1DDD 1E43 0261 0261 0261 | \
//!     0020 0020 0020 0020 0020 0020 0020 0020 | \
//!     0008 0002 0002 0002 0002 0002 0002 0002 |]");
//! # }
//! ```
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;
use std::fmt;
use std::ops::Deref;

mod table;
mod multistage;

pub struct SortKey(Vec<u16>);
pub use table::CollationTable;

impl Deref for SortKey {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for weight in &self.0 {
            if *weight == 0 {
                write!(f, "| ")?;
            } else {
                write!(f, "{:04X} ", weight)?;
            }
        }
        write!(f, "|")?;
        write!(f, "]")?;
        Ok(())
    }
}


pub fn collate(text: &str, table: &table::CollationTable) -> SortKey {
    let mut weights = Vec::new();
    for c in text.nfd() {
        weights.extend(table.resolve(c));
    }

    let mut sort_key = Vec::with_capacity(weights.len());
    // For all levels
    for level in 0..4 {
        for entry in &weights {
            let weight = entry.weights[level];
            if weight != 0 {
                sort_key.push(weight);
            }
        }
        sort_key.push(0);
    }
    while sort_key.last() == Some(&0) {
        sort_key.pop();
    }
    
    SortKey(sort_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::{char, u32};

    #[test]
    fn test() {
        let table = CollationTable::from_text_file("data/allkeys.txt");

        let file = File::open("data/CollationTest/CollationTest_NON_IGNORABLE.txt").unwrap();
        let file = BufReader::new(&file);
        for (line_num, line) in file.lines().enumerate() {
            let line = line.unwrap();
            let line = line.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let mut parts = line.split(';');

            let codes = parts.next().unwrap();
            let text = codes
                .split(" ")
                .map(|s| u32::from_str_radix(s, 16).unwrap())
                .map(|c| char::from_u32(c).unwrap())
                .collect::<String>();

            let sort_key = collate(&text, &table);

            let comment = parts.next().unwrap();
            let from = comment.find('[').unwrap();
            let to = comment.rfind(']').unwrap();
            let expected = &comment[from..to + 1];

            let actual = format!("{:?}", sort_key);
            assert_eq!(expected, actual, "failed on line '{}': {}", line_num, line);
        }
    }
}
