//! User interface module: functions for handling input/output and ANSI text display.
use std::io::{stdin, stdout, Write};

// Make the ANSI Art module public
pub mod ansi_art;

/// Print the title banner ASCII art.
pub fn show_title() {
    println!("{}", ansi_art::TITLE_BANNER);
}

/// Prompt the user for input, displaying a message, and return the input string.
pub fn prompt(message: &str) -> String {
    print!("{}", message);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim_end_matches(['\n', '\r']).to_string();
    clear_screen();
    input
}

/// Clear the terminal screen (if supported) and move cursor to home.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    let _ = stdout().flush();
}
