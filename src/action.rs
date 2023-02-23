use serde::{Deserialize, Serialize};

use crate::tile::Tile;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    GiveClue { clue: String },
    SetVote { tile: Tile, vote: bool },
    TapTile { tile: Tile },
}
