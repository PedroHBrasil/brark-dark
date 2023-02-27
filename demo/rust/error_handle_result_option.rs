use std::fs::File;
use std::io::{Error, Read};

fn read_file_contents(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_word(contents: &str, word: &str) -> Option<usize> {
    contents.find(word)
}

fn main() {
    let contents = match read_file_contents("example.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let word = "rust";
    match find_word(&contents, word) {
        Some(index) => println!("Found the word '{}' at index {}", word, index),
        None => println!("Could not find the word '{}'", word),
    }
}
