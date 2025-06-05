pub mod data;
pub mod utils;

use std::rc::Rc;

use data::Data;
use thiserror::Error as TError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub guesses: Vec<String>,
    pub word: String,
    pub data: Rc<Data>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum CharacterState {
    Correct,
    WrongPosition,
    NotFound,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct BoardStateCharacter(pub char, pub Option<CharacterState>);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct BoardState(pub Vec<Vec<BoardStateCharacter>>);

pub trait WinTrait {
    fn has_won(&self) -> bool;
}

impl WinTrait for BoardState {
    fn has_won(&self) -> bool {
        for guess in &self.0 {
            let mut failed = false;
            for character in guess {
                if character.1 != Some(CharacterState::Correct) {
                    failed = true;
                }
            }
            if !failed {
                return true;
            }
        }

        false
    }
}

impl WinTrait for Board {
    fn has_won(&self) -> bool {
        self.guesses.contains(&self.word)
    }
}

impl Into<BoardState> for &Board {
    fn into(self) -> BoardState {
        BoardState(
            self.guesses
                .iter()
                .map(|word| {
                    word.chars()
                        .enumerate()
                        .map(|(index, character)| {
                            let word_chars = self.word.chars();
                            let state = if let Some((i, _)) =
                                word_chars.enumerate().find(|x| &x.1 == &character)
                            {
                                if i == index {
                                    CharacterState::Correct
                                } else {
                                    CharacterState::WrongPosition
                                }
                            } else {
                                CharacterState::NotFound
                            };

                            BoardStateCharacter(character, Some(state))
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new_with_length(5)
    }
}

impl Board {
    pub fn new_empty_board() -> Self {
        let data: Data = Data::default();
        Self {
            guesses: Vec::new(),
            data: Rc::new(data),
            word: String::new(),
        }
    }

    pub fn get_key_state(&self, c: char) -> Option<CharacterState> {
        let mut best = None;
        for guess in self.get_state().0 {
            for c in guess.iter().filter(|x| x.0 == c.to_ascii_lowercase()) {
                match c.1 {
                    Some(CharacterState::Correct) | Some(CharacterState::NotFound) => {
                        return c.1.clone();
                    }
                    Some(CharacterState::WrongPosition) => best = c.1.clone(),
                    _ => {}
                }
            }
        }

        best
    }

    pub fn new_with_word(word: String) -> Self {
        let mut board = Self::new_empty_board();
        board.reset_with_word(word);
        board
    }

    pub fn reset(&mut self) {
        self.reset_with_word(self.data.get_random_word(self.word.len()));
    }

    pub fn reset_with_word(&mut self, word: String) {
        self.word = word.to_lowercase();
        self.guesses.clear();
    }

    pub fn new_with_length(len: usize) -> Self {
        let mut board = Self::new_empty_board();
        board.reset_with_length(len);
        board
    }

    pub fn reset_with_length(&mut self, len: usize) {
        let word = self.data.get_random_word(len);
        self.word = word;
        self.guesses.clear();
    }

    pub fn get_state(&self) -> BoardState {
        self.into()
    }

    pub fn add_guess<T: Into<String>>(&mut self, input: T) -> Result<(), GuessError> {
        let input: String = input.into();
        if let Some(_) = self.data.0.get(&input.to_lowercase()) {
            if input.len() == self.word.len() {
                self.guesses.push(input.to_lowercase());
            } else {
                return Err(GuessError::WrongSize);
            }
        } else {
            return Err(GuessError::NotFound);
        }

        Ok(())
    }
}

#[derive(Debug, TError, Clone)]
pub enum GuessError {
    #[error("Wrong Size")]
    WrongSize,
    #[error("Not Found")]
    NotFound,
}
