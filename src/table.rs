use pest::{self, Parser};
use multistage::MultiStage;
use std::{usize, u16, u32};
use std::fs::File;
use std::io::Read;
use std::fmt;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct TableParser;

type Nodes<'a> = pest::iterators::Pairs<'a, Rule>;
type Node<'a> = pest::iterators::Pair<'a, Rule>;

pub struct WeightEntry {
    /// Weights for levels 1-4
    pub weights: [u16; 4],
}

impl fmt::Debug for WeightEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for weight in &self.weights {
            write!(f, ".{:04X}", weight)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub struct CollationTable {
    /// Weights for all entries
    weights: Vec<WeightEntry>,
    table: MultiStage<PrimaryEntry>,
}

impl CollationTable {
    fn new() -> Self {
        Self {
            weights: Vec::new(),
            table: MultiStage::new(8),
        }
    }

    pub fn from_text(data: &str) -> CollationTable {
        let mut table: Nodes = TableParser::parse(Rule::table, data).unwrap();
        let table = table.next().unwrap();
        let entries = table
            .into_inner()
            .skip_while(|p| p.as_rule() != Rule::entry);

        let mut max_contr = 0;

        let mut table = CollationTable::new();
        for entry in entries {
            let mut it = entry.into_inner();
            let mut codepoints = it.next().unwrap().into_inner();
            let elements = it.next().unwrap();

            // Record weights (FIXME: de-duplicate?)
            let weights_idx = table.weights.len();
            table.weights.extend(elements.into_inner().map(scan_weight));

            let first = u32::from_str_radix(codepoints.next().unwrap().as_str(), 16).unwrap();
            let entry = table.table.entry(first);

            if codepoints.next().is_some() {
                entry.contraction += 1;
                max_contr = max_contr.max(entry.contraction);
            } else {
                // Record entry's own weights
                debug_assert_eq!(entry.weights, usize::MAX);
                entry.weights = weights_idx;
                entry.len = (table.weights.len() - weights_idx) as u8;
            }
        }

        table
    }

    pub fn from_text_file(path: &str) -> CollationTable {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        CollationTable::from_text(&buf)
    }

    pub fn resolve(&self, c: char) -> &[WeightEntry] {
        self.table
            .get(c as u32)
            .and_then(|e| {
                if e.weights == usize::MAX {
                    None
                } else {
                    Some(&self.weights[e.weights..e.weights + (e.len as usize)])
                }
            })
            .unwrap_or(&[])
    }
}

/// Entry in the collation table corresponding to the first character
#[derive(Clone)]
struct PrimaryEntry {
    /// Index into weights array
    weights: usize,
    /// Length of weights block. If `len` is 0, this entry does not have assigned weights
    /// and is only used as a beginning of some contraction.
    len: u8,
    /// Index into contraction table, if this entry is a beginning of one or more contraction
    /// sequences. Set to `u16::MAX` if not a part of contraction.
    contraction: u16,
}

impl Default for PrimaryEntry {
    fn default() -> Self {
        Self {
            weights: usize::MAX,
            len: 0,
            contraction: 0,
        }
    }
}

fn scan_weight(node: Node) -> WeightEntry {
    debug_assert_eq!(node.as_rule(), Rule::element);

    let mut it = node.into_inner();
    let _alt = it.next().unwrap();

    let mut weights = [0; 4];
    for i in 0..4 {
        weights[i] = it.next()
            .map(|r| u16::from_str_radix(r.as_str(), 16).unwrap())
            .unwrap_or(0);
    }

    WeightEntry { weights }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse() {
        let _table = CollationTable::from_text_file("data/allkeys.txt");
    }
}
