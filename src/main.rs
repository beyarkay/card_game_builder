use serde::{Deserialize, Serialize};
use serde_yaml::{self};

fn main() {
    let f = std::fs::File::open("games/chameleon.yaml").expect("Could not open file.");
    let chameleon_game: Game = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:#?}", chameleon_game);
}

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    name: String,
    version: String,
    instructions: String,
    num_players: String,
    duration: String,
    authors: String,
    website: String,
    expansions: Vec<Expansion>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Expansion {
    name: String,
    categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    name: String,
    items: Vec<String>,
}
