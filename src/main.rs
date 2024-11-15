use colored::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::io::stdout;
use std::io::Write;
use std::iter::FromIterator;

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

struct TypingMonkey {
    wordlist: HashSet<String>,
    alphabet: Vec<char>,
    // internals
    char_buffer: VecDeque<char>,
    mark_buffer: VecDeque<bool>,
    rng: ThreadRng,
}

impl TypingMonkey {
    fn new(wordlist: HashSet<String>, alphabet: Vec<char>) -> TypingMonkey {
        let max_buffer_len = wordlist.iter().map(|w| w.len()).max().unwrap();

        let mut monkey = TypingMonkey {
            wordlist,
            alphabet,
            char_buffer: VecDeque::with_capacity(max_buffer_len),
            mark_buffer: VecDeque::with_capacity(max_buffer_len),
            rng: rand::thread_rng(),
        };

        // fill buffers
        for _ in 0..max_buffer_len {
            monkey.type_rand_char();
            monkey.mark_chars_part_of_words();
        }

        monkey
    }

    fn mark_chars_part_of_words(&mut self) {
        let end = self.char_buffer.len();
        let buffer_str: String = self.char_buffer.iter().collect();
        for start in 0..end {
            if self.wordlist.contains(&buffer_str[start..end]) {
                for idx in start..end {
                    self.mark_buffer[idx] = true;
                }
                return;
            }
        }
    }

    fn next_char(&mut self) -> (char, bool) {
        let char = self.char_buffer.pop_front().unwrap();
        let part_of_word = self.mark_buffer.pop_front().unwrap();
        (char, part_of_word)
    }

    fn type_rand_char(&mut self) {
        let c = self.gen_rand_char();
        self.char_buffer.push_back(c);
        self.mark_buffer.push_back(false);
    }

    fn gen_rand_char(&mut self) -> char {
        let idx = self.rng.gen_range(0..self.alphabet.len());
        self.alphabet[idx]
    }
}

fn main() -> Result<(), String> {
    let wordlist_path = "wordlist.txt";

    let wordlist: HashSet<String> = read_to_collection(wordlist_path)?;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut typing_monkey = TypingMonkey::new(wordlist, alphabet);

    loop {
        let (char, part_of_word) = typing_monkey.next_char();
        if part_of_word {
            print!("{}", char.to_string().yellow())
        } else {
            print!("{}", char)
        }
        stdout()
            .flush()
            .map_err(|e| format!("error flushing stdout: {}", e))?;
        typing_monkey.type_rand_char();
        typing_monkey.mark_chars_part_of_words();
        //thread::sleep(Duration::from_millis(50));
    }
}
