// Day 19 — CLI parsing with clap
// Describe your arguments as an annotated struct; clap generates the parser,
// validation, help text, and error messages. Feels like Spring annotations.
//
// Cargo.toml needs: cargo add clap --features derive
//
// Try:
//   cargo run -- --help
//   cargo run -- Aria
//   cargo run -- Aria --class mage --health 80 --verbose
//   cargo run -- Borin -c warrior

use clap::{Parser, ValueEnum};

// This struct IS the CLI definition. Each field = one argument.
#[derive(Parser, Debug)]
#[command(name = "character-creator")]
#[command(about = "Create a game character from the command line")]
struct Args {
    // POSITIONAL argument — no --flag, just the first bare value.
    // `cargo run -- Aria`  ->  name = "Aria"
    name: String,

    // OPTION with short + long + a default. Optional because it has a default.
    // -c mage  OR  --class mage
    #[arg(short, long, default_value = "rogue")]
    class: String,

    // OPTION taking a number, with a typed default. clap validates it's a u32.
    #[arg(short = 'H', long, default_value_t = 100)]
    health: u32,

    // FLAG — a boolean. Present = true, absent = false.
    #[arg(short, long)]
    verbose: bool,

    // OPTION restricted to specific values via an enum (see below).
    // clap rejects anything not in the list, automatically.
    #[arg(short, long, value_enum, default_value_t = Difficulty::Normal)]
    difficulty: Difficulty,

    // #[arg(short, long, default_value_t = 1)]
    // level: u32,

    // OPTION: starting gold, defaults to 0.
    #[arg(short, long, default_value_t = 0)]
    gold: u32,

    #[arg(short, long)]
    load: Option<String>,
}

// An enum used as an argument type — clap only accepts these exact values.
#[derive(ValueEnum, Clone, Debug)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

fn main() {
    // One line does all parsing. On bad input, clap prints an error + usage and exits.
    let args = Args::parse();

    println!("=== Character Created ===");
    println!("Name:       {}", args.name);
    println!("Class:      {}", args.class);
    println!("Health:     {}", args.health);
    println!("Difficulty: {:?}", args.difficulty);
    // println!("Level:      {}", args.level);
    println!("Gold:       {}", args.gold);

    match &args.load {
        Some(path) => println!("Loading from {}", path),
        None => println!("Starting a new game"),
    }

    // The flag changes behaviour:
    if args.verbose {
        println!("\n[verbose] Full args: {:?}", args);
    }

    
}