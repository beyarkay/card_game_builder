use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::io::Write;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    if args.len() != 2 {
        println!("Using `games/chameleon.yaml` as default gamefile.");
        filename = "games/chameleon.yaml".to_string();
    } else {
        filename = format!("{}", args[1]);
    }
    let f = std::fs::File::open(filename).expect("Could not open file.");
    let chameleon_game: Game = serde_yaml::from_reader(f).expect("Could not read values.");
    // Create one PDF per expansion
    for expansion in chameleon_game.expansions {
        println!("Writing expansion '{}'", expansion.name);
        let mut pdf = format!("\\documentclass[config, grid, fronts]{{flashcards}}\n\
                            \\cardfrontstyle[\\LARGE]{{headings}}\n\
                            \\begin{{document}}\n\
                            \t\\cardfrontfoot{{{0}}}\n\n", expansion.name);
        for category in expansion.categories {
            for item in category.items {
                pdf.push_str(build_card_snippet(
                        chameleon_game.name.as_str(),
                        item.as_str()
                        ).as_str());
            }
        }
        pdf.push_str("\n\\end{document}\n");
        let filename = format!("tex/{}: {}.tex", chameleon_game.name, expansion.name);
        let mut output = File::create(filename).expect("File could not be created");
        write!(output, "{}", pdf).expect("File could not be written to");
    }

}

fn build_card_snippet<'a>(game_name: &'a str, content: &'a str) -> String {
    // Curly braces are escaped by doubling them:
    return format!("\t\\begin{{flashcard}}[{0}] {{{1}}} \\end{{flashcard}}\n",
            game_name, content);
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
