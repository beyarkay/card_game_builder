use std::fs;

fn main() {
    let filename = "text_sources/so_youre_in_a_hot_air_balloon.txt";
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file.");

    for line in contents.split("\n") {
        if line.chars().count() ==  0 {
            continue;
        }
        if line.starts_with('#') {
            println!("Excluding Comment: {}", line)
        } else if line.starts_with('[') {
            let until = line.chars().count();
            let (tag, _bracket) = line.trim().split_at(1).1.split_at(until - 2);
            println!("Tag Starts: {}", tag)
        } else {
            //println!("- {}", line);
        }
    }

}
