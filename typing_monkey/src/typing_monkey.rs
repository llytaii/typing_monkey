use ahash::AHashSet;
use colored::*;
use rand::{rngs::ThreadRng, Rng};
use std::{collections::VecDeque, fs::OpenOptions, io::Write, time::Instant};

pub struct TypingMonkey<'a> {
    wordlist: AHashSet<String>,
    alphabet: Vec<char>,
    longest_words: Vec<String>,
    longest_words_path: &'a str,
    longest_words_count: usize,
    // internals
    buffer: VecDeque<(char, bool)>,
    rng: ThreadRng,
    buffer_str: String,
}

impl TypingMonkey<'_> {
    pub fn new(
        wordlist: AHashSet<String>,
        alphabet: Vec<char>,
        longest_words: Vec<String>,
        longest_words_path: &str,
        longest_words_count: usize,
    ) -> TypingMonkey {
        let max_buffer_len = wordlist.iter().map(|w| w.len()).max().unwrap();

        let mut monkey = TypingMonkey {
            wordlist,
            alphabet,
            longest_words,
            longest_words_path,
            longest_words_count,
            buffer: VecDeque::with_capacity(max_buffer_len),
            rng: rand::thread_rng(),
            buffer_str: String::with_capacity(max_buffer_len),
        };

        // fill buffers
        for _ in 0..max_buffer_len {
            monkey.type_rand_char();
            monkey.mark_chars_part_of_words();
        }

        monkey
    }

    pub fn benchmark_next_without_print(&mut self, iterations: usize) {
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = self.next_char();
            self.type_rand_char();
            self.mark_chars_part_of_words();
        }
        let duration = start.elapsed();
        println!("{} iterations took {:?}", iterations, duration);
    }

    pub fn print_next(&mut self) {
        let (char, part_of_word) = self.next_char();
        if part_of_word {
            print!("{}", char.to_string().yellow())
        } else {
            print!("{}", char)
        }
        let _ = std::io::stdout().flush();
        self.type_rand_char();
        self.mark_chars_part_of_words();
    }

    fn add_word_to_longest_words(&mut self, word: &str) -> bool {
        if self.longest_words.contains(&String::from(word)) {
            return false;
        }

        if self.longest_words.len() < self.longest_words_count {
            self.longest_words.push(String::from(word));
        } else {
            let s = self
                .longest_words
                .iter()
                .enumerate()
                .min_by_key(|(_, w)| w.len());

            if let Some((shortest_idx, _)) = s {
                if word.len() > self.longest_words[shortest_idx].len() {
                    self.longest_words[shortest_idx] = word.to_owned();
                }
            }
        }

        true
    }

    fn write_longest_words_to_file(&mut self) {
        self.longest_words
            .sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.longest_words_path);

        match file {
            Ok(mut file) => {
                let content = self
                    .longest_words
                    .iter()
                    .map(|word| format!("{}\n", word))
                    .collect::<String>();

                let _ = file.write_all(content.as_bytes());
            }
            _ => {}
        }
    }

    fn mark_chars_part_of_words(&mut self) {
        self.buffer_str.clear();
        for (c, _) in self.buffer.iter() {
            self.buffer_str.push(*c);
        }

        let end = self.buffer.len();
        for start in 0..end {
            if self.wordlist.contains(&self.buffer_str[start..end]) {
                for idx in start..end {
                    self.buffer[idx].1 = true;
                }

                let word = self.buffer_str[start..end].to_string();

                if self.add_word_to_longest_words(&word) {
                    self.write_longest_words_to_file();
                }

                return;
            }
        }
    }

    fn next_char(&mut self) -> (char, bool) {
        self.buffer.pop_front().unwrap()
    }

    fn type_rand_char(&mut self) {
        let c = self.gen_rand_char();
        self.buffer.push_back((c, false));
    }

    fn gen_rand_char(&mut self) -> char {
        let idx = self.rng.gen_range(0..self.alphabet.len());
        self.alphabet[idx]
    }
}
