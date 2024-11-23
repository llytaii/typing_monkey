use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

fn clear_console() {
    // ANSI escape code to clear the console
    print!("\x1B[2J\x1B[1;1H");
}

fn print_file_content(file_path: &PathBuf) {
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            clear_console();
            println!("longest words:\n{}", contents);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

fn main() {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file-path>", args[0]);
        std::process::exit(1);
    }
    let file_path = PathBuf::from(&args[1]);

    // Check if the file exists
    if !file_path.exists() {
        eprintln!("error: file '{}' does not exist.", file_path.display());
        std::process::exit(1);
    }

    // Print the file's content once at startup
    print_file_content(&file_path);

    // Create a debouncer with a callback
    let file_path_for_closure = file_path.clone();
    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                if events.iter().any(|e| e.event.kind.is_modify()) {
                    print_file_content(&file_path_for_closure);
                }
            }
            Err(err) => {
                eprintln!("watch error: {:?}", err);
            }
        },
    )
    .expect("failed to create debouncer");

    // Watch the specified file
    debouncer
        .watch(&file_path, RecursiveMode::NonRecursive)
        .expect("failed to watch file");

    loop {
        std::thread::park();
    }
}
