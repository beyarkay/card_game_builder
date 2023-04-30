use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{env, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("No arguments provided. Usage: `cargo run --release games/*.yaml`")
    }
    for (i, filename) in args.iter().skip(1).enumerate() {
        println!("[{}/{}] Processing {}", i + 1, args.len() - 1, filename);
        process_file(filename.to_string())?;
    }
    Ok(())
}

fn process_file(filename: String) -> Result<(), Box<dyn Error>> {
    let todays_date = format!("{}", Utc::now().format("%e %B %Y"));
    let f = std::fs::File::open(filename)?;
    let game: Game = serde_yaml::from_reader(f)?;
    let mut file_template = std::fs::read_to_string("template.tex")?;
    // Create one PDF per expansion
    for expansion in game.expansions {
        println!("Writing expansion '{}'...", expansion.name);
        file_template = file_template
            .replace("{game_name}", &escape_latex(game.name.as_str()))
            .replace("{expansion_name}", &escape_latex(expansion.name.as_str()))
            .replace("{authors}", &escape_latex(&game.authors))
            .replace("{website}", &escape_latex(&game.website))
            .replace("{duration}", &escape_latex(&game.duration))
            .replace("{date}", &todays_date)
            .replace("{instructions}", &md_to_latex(&game.instructions))
            .replace("{num_players}", &escape_latex(&game.num_players));
        let mut flashcards = "".to_string();
        for category in expansion.categories {
            for item in category.items {
                let escaped = escape_latex(item.as_str());
                let item = match item.len() {
                    520.. => format!("\\small{{{}}}", escaped),
                    360..=519 => format!("\\normalsize{{{}}}", escaped),
                    220..=359 => format!("\\large{{{}}}", escaped),
                    150..=219 => format!("\\Large{{{}}}", escaped),
                    // The default size for the flashcards is `\LARGE{}`
                    _ => escaped.to_string(),
                };
                flashcards.push_str(
                    format!(
                        r"    \begin{{flashcard}}[\color{{lightgray}}{{{game_name} ({category_name})}}] {{{flashcard_content}}} \end{{flashcard}}{newline}",
                        game_name=escape_latex(game.name.as_str()),
                        category_name=escape_latex(category.name.as_str()),
                        flashcard_content=item,
                        newline="\n"
                    )
                    .as_str(),
                );
                // break;
            }
            // break;
        }
        // Fill in all the flashcards
        file_template = file_template.replace("{flashcard_items}", &flashcards);

        // Create a directory which the .tex files can be written to
        fs::create_dir_all("tex")?;
        // Figure out what the filename will be
        let filename = format!("tex/{}_{}.tex", game.name, expansion.name).replace(' ', "_");
        // Write the game to the tex file
        let mut output = File::create(filename.as_str())?;
        // Actually write out the file
        write!(output, "{}", file_template)?;
        println!(
            "Expansion {} successfully written as {}",
            expansion.name, filename
        );
        // Now write the tex file to pdf
        if cfg!(target_os = "windows") {
            println!("Sorry, I don't know how to convert .tex files to .pdf files on windows");
        } else {
            fs::create_dir_all("pdfs")?;
            let out = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "pdflatex --output-dir=pdfs {}",
                    filename.replace(' ', "-")
                ))
                .output()?;
            io::stdout().write_all(&out.stdout)?;
            io::stdout().write_all(&out.stderr)?;

            // also clean up the LaTeX build files
            let _cleanup = Command::new("sh")
                .arg("-c")
                .arg("rm pdfs/*.out pdfs/*.aux pdfs/*.log")
                .output() // Output is smothered
                ?;
        }
    }
    Ok(())
}

fn md_to_latex(s: &str) -> String {
    let header_re = Regex::new(r"\n?\s*# (.+)\n?").unwrap();
    let newline_re = Regex::new(r"\n").unwrap();
    escape_latex(&newline_re.replace_all(
        &header_re.replace_all(s, r" \subsubsection*{$1} "),
        r" \par ",
    ))
}

fn escape_latex(s: &str) -> String {
    s.replace('_', "\\_")
        .replace('^', "\\^")
        .replace('&', "\\&")
        .replace('$', "\\$")
        .replace('%', "\\%")
        .replace('#', "\\#")
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
