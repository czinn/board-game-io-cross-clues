use serde::{Deserialize, Serialize};

use crate::tile::Tile;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub size: Tile,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: Tile { row: 4, col: 4 },
        }
    }
}
