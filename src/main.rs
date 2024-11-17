use ahash::AHashSet;
use std::fs;
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;
use typing_monkey::TypingMonkey;

mod typing_monkey;

fn read_to_collection<T>(file_path: &str) -> Result<T, String>
where
    T: FromIterator<String>,
{
    let collection = fs::read_to_string(file_path)
        .map_err(|e| format!("error reading file {}: {}", file_path, e))?
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<T>();
    Ok(collection)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let mut typing_delay_ms = 50;
    if args.len() > 1 {
        typing_delay_ms = match args[1].parse() {
            Ok(val) => val,
            Err(_) => typing_delay_ms,
        };
    }

    let wordlist_path = "wordlist.txt";
    let longest_words_path = "longest_words.txt";

    let wordlist: AHashSet<String> = read_to_collection(wordlist_path)?;
    let longest_words: Vec<String> = read_to_collection(longest_words_path)?;

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut typing_monkey =
        TypingMonkey::new(wordlist, alphabet, longest_words, longest_words_path, 5);

    loop {
        typing_monkey.print_next();
        thread::sleep(Duration::from_millis(typing_delay_ms));
    }
}

#[test]
fn test() -> Result<(), String> {
    let wordlist_path = "wordlist.txt";
    let longest_words_path = "longest_words.txt";

    let wordlist: AHashSet<String> = read_to_collection(wordlist_path)?;
    let longest_words: Vec<String> = read_to_collection(longest_words_path)?;

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut typing_monkey =
        TypingMonkey::new(wordlist, alphabet, longest_words, longest_words_path, 5);

    typing_monkey.benchmark_next_without_print(10_000_000);

    Ok(())
}
