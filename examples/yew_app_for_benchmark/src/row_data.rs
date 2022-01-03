use rand::prelude::*;

use crate::constants::ADJECTIVES;
use crate::constants::COLOURS;
use crate::constants::NOUNS;

#[derive(Clone, PartialEq)]
pub struct RowData {
    pub id: usize,
    pub label: String,
}

impl RowData {
    pub fn new(id: usize, rng: &mut SmallRng) -> Self {
        let mut label = String::new();
        label.push_str(ADJECTIVES.choose(rng).unwrap());
        label.push(' ');
        label.push_str(COLOURS.choose(rng).unwrap());
        label.push(' ');
        label.push_str(NOUNS.choose(rng).unwrap());

        Self { id, label }
    }
}
