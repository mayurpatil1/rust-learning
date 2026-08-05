// The scoreboard: data + persistence + queries.
use serde::{Deserialize, Serialize};
use std::fs;

use crate::error::ScoreError;

const FILE: &str = "scores.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScoreBoard {
    pub entries: Vec<Entry>,
}

impl ScoreBoard {
    // Load from disk. A missing file is NOT an error — it just means "empty board".
    pub fn load() -> Result<ScoreBoard, ScoreError> {
        match fs::read_to_string(FILE) {
            Ok(json) => {
                // Parsing can fail -> map that into our Corrupt error, then ? it.
                let board = serde_json::from_str(&json)
                    .map_err(|e| ScoreError::Corrupt(e.to_string()))?;
                Ok(board)
            }
            Err(_) => Ok(ScoreBoard::default()), // no file yet -> fresh board
        }
    }

    // Save to disk. Both serialization and writing can fail.
    pub fn save(&self) -> Result<(), ScoreError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ScoreError::Write(e.to_string()))?;
        fs::write(FILE, json).map_err(|e| ScoreError::Write(e.to_string()))?;
        Ok(())
    }

    pub fn add(&mut self, name: String, score: u32) {
        self.entries.push(Entry { name, score });
    }

    // Return entries sorted high-to-low (Day 5 iterators + sort).
    pub fn ranked(&self) -> Vec<&Entry> {
        let mut list: Vec<&Entry> = self.entries.iter().collect();
        list.sort_by(|a, b| b.score.cmp(&a.score)); // b vs a = descending
        list
    }

    // The single highest score, or an Empty error if there are none.
    pub fn top(&self) -> Result<&Entry, ScoreError> {
        self.entries
            .iter()
            .max_by_key(|e| e.score)
            .ok_or(ScoreError::Empty) // Option -> Result
    }
}