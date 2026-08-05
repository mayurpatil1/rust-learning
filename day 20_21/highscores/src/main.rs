// Days 20-21 — Consolidation: a high-score CLI tool.
// Combines clap subcommands, thiserror, serde, the ? operator, and iterators.

mod error;
mod scores;

use clap::{Parser, Subcommand};

use error::ScoreError;
use scores::ScoreBoard;

#[derive(Parser)]
#[command(name = "highscores", about = "Track game high scores")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// Each variant is a SUBCOMMAND; its fields are that subcommand's arguments.
// (Day 12 data-carrying enums meet Day 19 clap.)
#[derive(Subcommand)]
enum Command {
    /// Record a new score
    Add { name: String, score: u32 },
    /// List all scores, highest first
    List,
    /// Show the top score
    Top,
}

// All the real work lives here and returns Result, so ? works throughout.
fn run() -> Result<(), ScoreError> {
    let cli = Cli::parse();
    let mut board = ScoreBoard::load()?; // load, propagating any error

    match cli.command {
        Command::Add { name, score } => {
            board.add(name.clone(), score);
            board.save()?; // propagate save errors
            println!("Recorded {} with {} points.", name, score);
        }
        Command::List => {
            let ranked = board.ranked();
            if ranked.is_empty() {
                println!("No scores yet.");
            } else {
                println!("=== High Scores ===");
                for (i, entry) in ranked.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, entry.name, entry.score);
                }
            }
        }
        Command::Top => {
            let best = board.top()?; // returns Empty error if none
            println!("Top score: {} with {} points!", best.name, best.score);
        }
    }

    Ok(())
}

fn main() {
    // main's ONE job: run the program and report any error cleanly.
    // No panics, no stack traces — a friendly message and a proper exit code.
    if let Err(e) = run() {
        eprintln!("Error: {}", e); // eprintln = print to stderr
        std::process::exit(1);
    }
}