use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, HashSet};

use board_game_io_base::error::Error;
use board_game_io_base::game::Game as GameTrait;
use board_game_io_base::ids::PlayerId;
use board_game_io_base::result::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::action::Action;
use crate::config::Config;
use crate::tile::Tile;
use crate::words::get_word_lists;

#[derive(Serialize, Deserialize, Clone)]
pub struct Clue {
    player: PlayerId,
    tile: Option<Tile>, // sometimes hidden
    clue: String,
    guess: Option<Tile>,
    guesser: Option<PlayerId>,
}

impl Clue {
    fn hide_tile(&self) -> Self {
        Self {
            tile: None,
            ..self.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    size: Tile,
    labels: Vec<String>, // first rows, then columns
    players: Vec<PlayerId>,
    player_tiles: BTreeMap<PlayerId, Option<Tile>>,
    current_clue: Option<Clue>,
    tiles: Vec<Tile>,
    votes: Vec<Vec<Vec<PlayerId>>>,
    good_clues: Vec<Vec<Option<Clue>>>,
    bad_clues: Vec<Clue>,
}

impl Game {
    fn get_player_tile(&mut self, player: &PlayerId) -> Result<&mut Option<Tile>> {
        match self.player_tiles.get_mut(&player) {
            Some(tile) => Ok(tile),
            None => Err(Error::InvalidAction("player does not exist".into())),
        }
    }

    fn game_over(&self) -> bool {
        self.tiles.len() == 0 && self.player_tiles.values().all(|tile| tile.is_none())
    }
}

#[derive(Serialize)]
pub struct View<'a> {
    size: &'a Tile,
    labels: &'a Vec<String>,
    players: &'a Vec<PlayerId>,
    players_with_tiles: Vec<PlayerId>,
    player_tile: Option<Tile>,
    current_clue: Option<Clue>,
    tiles_remaining: usize,
    votes: &'a Vec<Vec<Vec<PlayerId>>>,
    good_clues: &'a Vec<Vec<Option<Clue>>>,
    bad_clues: Vec<Clue>,
}

impl GameTrait for Game {
    type View<'a> = View<'a>;
    type Action = Action;
    type Config = Config;

    fn new(
        Config {
            size,
            word_lists,
            custom_words,
        }: Config,
        num_players: u32,
    ) -> Result<Self> {
        let players: Vec<PlayerId> = (0..num_players).map(|x| PlayerId(x)).collect();
        let mut tiles: Vec<Tile> = (0..size.row)
            .flat_map(|row| (0..size.col).map(move |col| Tile { row, col }))
            .collect();
        let good_clues: Vec<Vec<Option<Clue>>> =
            (0..size.row).map(|_row| vec![None; size.col]).collect();
        let votes: Vec<Vec<Vec<PlayerId>>> = (0..size.row)
            .map(|_row| (0..size.col).map(|_col| Vec::new()).collect())
            .collect();
        tiles.shuffle(&mut thread_rng());
        let player_tiles: BTreeMap<PlayerId, Option<Tile>> =
            players.iter().map(|p| (*p, tiles.pop())).collect();
        let all_word_lists = get_word_lists();
        let mut word_set: HashSet<String> = word_lists
            .into_iter()
            .filter_map(|(key, enabled)| if enabled { Some(key) } else { None })
            .flat_map(|key| {
                all_word_lists
                    .get(&key)
                    .unwrap()
                    .into_iter()
                    .map(|w| w.to_string().clone())
            })
            .collect();
        word_set.extend(custom_words.into_iter());
        let words: Vec<String> = word_set.into_iter().collect();
        if words.len() < size.row + size.col {
            return Err(Error::InvalidCreate);
        }
        let labels: Vec<String> = words
            .choose_multiple(&mut thread_rng(), size.row + size.col)
            .map(|w| w.clone())
            .collect();
        Ok(Game {
            size,
            labels,
            players,
            player_tiles,
            current_clue: None,
            tiles,
            votes,
            good_clues,
            bad_clues: Vec::new(),
        })
    }

    fn players(&self) -> Vec<PlayerId> {
        self.players.clone()
    }

    #[instrument(skip_all)]
    fn view<'a>(&'a self, player: Option<PlayerId>) -> Self::View<'a> {
        let Self {
            size,
            labels,
            players,
            player_tiles,
            current_clue,
            tiles,
            votes,
            good_clues,
            bad_clues,
        } = &self;
        let current_clue = match current_clue {
            None => None,
            Some(clue) => {
                if Some(clue.player) == player {
                    Some(clue.clone())
                } else {
                    Some(clue.hide_tile())
                }
            }
        };
        let players_with_tiles = player_tiles
            .iter()
            .filter_map(|(p, t)| if t.is_some() { Some(*p) } else { None })
            .collect();
        let player_tile = match player {
            Some(player) => player_tiles.get(&player).map(|o| o.clone()).flatten(),
            None => None,
        };
        let bad_clues = if self.game_over() {
            bad_clues.clone()
        } else {
            bad_clues.iter().map(|clue| clue.hide_tile()).collect()
        };
        Self::View {
            size: &size,
            labels: &labels,
            players: &players,
            players_with_tiles,
            player_tile,
            current_clue,
            tiles_remaining: tiles.len(),
            votes: &votes,
            good_clues: &good_clues,
            bad_clues,
        }
    }

    fn do_action(&mut self, player: PlayerId, action: &Action) -> Result<()> {
        match action {
            Action::GiveClue { clue } => {
                if self.current_clue.is_some() {
                    return Err(Error::InvalidAction("clue already given".into()));
                }

                let player_tile = match self.get_player_tile(&player)? {
                    Some(tile) => tile.clone(),
                    None => {
                        return Err(Error::InvalidAction(
                            "player does not have a tile to clue".into(),
                        ))
                    }
                };

                self.current_clue = Some(Clue {
                    player,
                    tile: Some(player_tile),
                    clue: clue.clone(),
                    guess: None,
                    guesser: None,
                });
                Ok(())
            }
            Action::SetVote { tile, vote } => {
                if self.good_clues[tile.row][tile.col].is_some() {
                    return Err(Error::InvalidAction("tile is not open".into()));
                }
                if let Some(clue) = &self.current_clue {
                    if clue.player == player {
                        return Err(Error::InvalidAction("cannot vote on own clue".into()));
                    }
                } else {
                    return Err(Error::InvalidAction("no active clue".into()));
                }

                let votes = &mut self.votes[tile.row][tile.col];
                let my_vote_index = votes.iter().position(|p| *p == player);
                match (my_vote_index, vote) {
                    (Some(_), true) => (),
                    (None, false) => (),
                    (Some(i), false) => {
                        votes.remove(i);
                        ()
                    }
                    (None, true) => votes.push(player),
                }
                Ok(())
            }
            Action::TapTile { tile } => {
                let mut current_clue = match self.current_clue.take() {
                    Some(clue) => clue,
                    None => return Err(Error::InvalidAction("no active clue".into())),
                };
                if self.good_clues[tile.row][tile.col].is_some() {
                    self.current_clue.insert(current_clue);
                    return Err(Error::InvalidAction("tile is not open".into()));
                }
                if current_clue.player == player {
                    self.current_clue.insert(current_clue);
                    return Err(Error::InvalidAction("cannot guess for own clue".into()));
                }
                // action is valid
                for vote_row in self.votes.iter_mut() {
                    for vote_cell in vote_row.iter_mut() {
                        vote_cell.clear();
                    }
                }
                let new_tile = self.tiles.pop();
                let player_tile = self.get_player_tile(&current_clue.player).unwrap();
                *player_tile = new_tile;
                current_clue.guess = Some(tile.clone());
                current_clue.guesser = Some(player);
                if current_clue.tile == current_clue.guess {
                    let _inserted = self.good_clues[tile.row][tile.col].insert(current_clue);
                } else {
                    self.bad_clues.push(current_clue);
                }
                Ok(())
            }
        }
    }
}
