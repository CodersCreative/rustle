pub mod data;
pub mod utils;

use data::Data;
use thiserror::Error as TError;

pub struct Board {
    pub guesses: Vec<String>,
    pub word: String,
    pub data: Data,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CharacterState {
    Correct,
    WrongPosition,
    NotFound,
}

pub struct BoardStateCharacter(pub char, pub CharacterState);

pub struct BoardState(pub Vec<Vec<BoardStateCharacter>>);

pub trait WinTrait {
    fn has_won(&self) -> bool;
}

impl WinTrait for BoardState {
    fn has_won(&self) -> bool {
        for guess in &self.0 {
            let mut failed = false;
            for character in guess {
                if character.1 != CharacterState::Correct {
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

                            BoardStateCharacter(character, state)
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
            data,
            word: String::new(),
        }
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
        self.data.0.insert(word.clone(), 1);
        self.word = word;
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
        if let Some(_) = self.data.0.get(&input) {
            if input.len() == self.word.len() {
                self.guesses.push(input);
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
