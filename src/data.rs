use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{collections::HashMap, error::Error, fs::File, io::Read};

pub const WORDS_PATH: &str = "./words.json";
pub const WORDS_SAVED_PATH: &str = "./words_saved.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data(pub HashMap<String, usize>);

impl Default for Data {
    fn default() -> Self {
        // return Self(WORDS);
        if let Ok(data) = Self::load(WORDS_SAVED_PATH) {
            data
        } else {
            Self::from_path(WORDS_PATH).unwrap()
        }
    }
}

impl Data {
    pub fn get_random_word(&self, len: usize) -> String {
        return self
            .0
            .iter()
            .filter(|(k, _)| k.len() == len)
            .nth(rand::random_range(0..len))
            .unwrap()
            .0
            .to_string();
    }

    fn from_json(json_str: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self(from_str(json_str)?))
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = File::open(path)?;
        let mut json_str = String::new();

        let _ = reader.read_to_string(&mut json_str)?;
        Self::from_json(&json_str)
    }

    pub fn save(&self, path: Option<&str>) {
        let path = match path {
            Some(p) => p,
            None => WORDS_SAVED_PATH,
        };
        let writer = File::create(path);

        if let Ok(writer) = writer {
            let _ = serde_json::to_writer_pretty(writer, &self);
        }
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = File::open(path)?;

        let mut data = String::new();
        let _ = reader
            .read_to_string(&mut data)
            .map_err(|e| e.to_string())?;

        return Ok(serde_json::from_str(&data)?);
    }
}
