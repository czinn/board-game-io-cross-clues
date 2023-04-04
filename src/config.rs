use serde::{Deserialize, Serialize};

use crate::tile::Tile;
use crate::words::get_word_lists;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub size: Tile,
    pub word_lists: Vec<(String, bool)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: Tile { row: 4, col: 4 },
            word_lists: get_word_lists().iter().enumerate().map(|(i, (key, _value))| (key.clone(), i == 0)).collect(),
        }
    }
}
