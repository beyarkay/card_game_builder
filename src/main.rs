use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, filename) in args.iter().skip(1).enumerate() {
        println!("[{}/{}] Processing {}", i + 1, args.len() - 1, filename);
        process_file(filename.to_string());
    }
}

fn process_file(filename: String) {
    let f = std::fs::File::open(filename).expect("Could not open file.");
    let game: Game = serde_yaml::from_reader(f).expect("Could not read values.");
    // Create one PDF per expansion
    for expansion in game.expansions {
        println!("Writing expansion '{}'...", expansion.name);
        let mut pdf = format!(
            "\\documentclass[playing_cards, grid, fronts]{{flashcards}}\n\
                              \\cardbackstyle{{empty}}\n\
                              \\cardfrontstyle[\\LARGE]{{headings}}\n\
                              \\begin{{document}}\n\
                              \t\\cardfrontfoot{{{0}}}\n\n",
            escape_latex(expansion.name.as_str())
        );
        for category in expansion.categories {
            for item in category.items {
                pdf.push_str(
                    format!(
                        "\t\\begin{{flashcard}}[{0} ({1})] {{{2}}} \\end{{flashcard}}\n",
                        escape_latex(game.name.as_str()),
                        escape_latex(category.name.as_str()),
                        escape_latex(item.as_str())
                    )
                    .as_str(),
                );
            }
        }
        pdf.push_str("\n\\end{document}\n");
        let filename = format!("tex/{}_{}.tex", game.name, expansion.name).replace(' ', "_");
        let mut output = File::create(filename.as_str()).expect("File could not be created");
        write!(output, "{}", pdf).expect("File could not be written to");
        println!(
            "Expansion {} successfully written as {}",
            expansion.name, filename
        );
        // Now write the tex file to pdf
        if cfg!(target_os = "windows") {
            println!("Sorry, I don't know how to convert .tex files to .pdf files on windows");
        } else {
            let out = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "pdflatex --output-dir=pdfs {}",
                    filename.replace(' ', "-")
                ))
                .output()
                .expect("Failed to compile pdf");
            io::stdout().write_all(&out.stdout).unwrap();
            io::stdout().write_all(&out.stderr).unwrap();

            // also clean up the LaTeX build files
            let _cleanup = Command::new("sh")
                .arg("-c")
                .arg("rm pdfs/*.out pdfs/*.aux pdfs/*.log")
                .output() // Output is smothered
                .expect("Failed to clean up LaTeX files");
        }
    }
}

fn escape_latex(s: &str) -> String {
    s.replace('_', "\\_")
        .replace('^', "\\^")
        .replace('&', "\\&")
        .replace('$', "\\$")
        .replace('%', "\\%")
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
