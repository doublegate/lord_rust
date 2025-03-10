//! # User Interface Module
//!
//! This module provides functions for handling user input/output and ANSI text display.
//! It manages the text-based interface of the game, including prompts, screen clearing,
//! and ASCII art display.
//!
//! ## Features
//!
//! - User input handling with prompts  
//! - Terminal screen management  
//! - ANSI color and art display  
//!
//! ## Implementation Details
//!
//! The module uses standard Rust I/O operations for terminal interaction.
//! ANSI escape sequences are used for text formatting and screen control.
//! The `ansi_art` submodule contains ASCII art for various game scenes.
use std::io::{stdin, stdout, Write};

/// ANSI art and colored text banners for the game
pub mod ansi_art;

/// Print the title banner ASCII art.
/// 
/// Displays the game's title screen with ANSI colors and ASCII art.
/// This is typically called at the start of the game to welcome the player.
pub fn show_title() {
    println!("{}", ansi_art::TITLE_BANNER);
}

/// Prompt the user for input, displaying a message, and return the input string.
/// 
/// This function:
/// 1. Displays the provided message
/// 2. Flushes stdout to ensure the message is shown
/// 3. Reads a line of input from the user
/// 4. Trims trailing newlines and carriage returns
/// 5. Clears the screen after input (for a clean UI)
/// 
/// # Arguments
/// 
/// * `message` - The prompt message to display to the user
/// 
/// # Returns
/// 
/// The user's input as a String, with trailing newlines removed
/// 
/// # Panics
/// 
/// Exits the program if reading from stdin fails
pub fn prompt(message: &str) -> String {
    print!("{}", message);
    let _ = stdout().flush();
    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input.");
        std::process::exit(1);
    }
    let input = input.trim_end_matches(['\n', '\r']).to_string();
    clear_screen();
    input
}

/// Clear the terminal screen (if supported) and move cursor to home position.
/// 
/// Uses ANSI escape sequences to:
/// - \x1B[2J: Clear the entire screen
/// - \x1B[H: Move cursor to home position (top-left)
/// 
/// This function is used throughout the game to maintain a clean interface
/// between different screens and prompts.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    let _ = stdout().flush();
}
