//! # Legend of the Red Dragon (Rust Edition)
//!
//! This is a modern Rust implementation of the classic BBS door game "Legend of the Red Dragon" (LORD).
//! The game features a text-based RPG experience with character progression, combat, and social interactions.
//!
//! ## Features
//!
//! - Player account creation and authentication  
//! - Persistent game state using PostgreSQL database  
//! - Daily game resets and player revival  
//! - Text-based user interface with ANSI color support  
//! - Multiple game areas: Town, Forest, PvP arena, and Romance options  
//!
//! ## Implementation Details
//!
//! The application is structured into three main modules:  
//! - `db`: Handles database operations including player data persistence and daily resets  
//! - `game`: Contains the game logic for different areas (town, forest, pvp, romance)  
//! - `ui`: Provides user interface utilities for display and input  
//!
//! The game uses Tokio for asynchronous operations, particularly for database access,
//! while maintaining a synchronous interface for the main game loop.

/// Database module for player persistence and game state management
mod db;
/// Game logic module containing gameplay mechanics and player interactions
mod game;
/// User interface module for display and input handling
mod ui;

use crate::db::verify_password;
use chrono::Local;
use tokio;

/// Main entry point for the Legend of the Red Dragon game.
/// 
/// This function:
/// 1. Initializes the database connection
/// 2. Performs daily reset operations if needed
/// 3. Displays the game title and welcome message
/// 4. Handles player authentication (login or account creation)
/// 5. Launches the main game loop
/// 6. Saves player data on exit
#[tokio::main]
async fn main() {
    // Clear the terminal screen for a clean start
    // This ensures the game UI begins with a fresh display
    ui::clear_screen();

    // Initialize the PostgreSQL database connection pool and apply schema if needed
    // This establishes the connection to the database and ensures all required tables exist
    // The connection pool is used throughout the application for all database operations
    let conn = match db::init_db_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    // Perform daily reset operations if a new day has started since last reset
    // This includes resetting forest fights, reviving dead players, restoring health, etc.
    if let Err(e) = db::daily_reset(&conn).await {
        eprintln!("Failed to perform daily reset: {}", e);
    }

    // Display the game title with ANSI art for visual appeal
    ui::show_title();
    println!("*** Welcome to \x1B[1mLegend of the Red Dragon\x1B[0m (Rust Edition)! ***");
    println!("  By DoubleGate -+-+-+- ver.0.2.0 -+-+-+- March 8th, 2025");
    println!(); // blank line for better readability

    // User authentication loop: continues until a valid login occurs (either an existing user logs in or a new account is created)
    let player = loop {
        // Prompt for player name
        let name_input = ui::prompt("Enter your name: ");
        let name = name_input.trim().to_string();

        // Validate user input for name
        if name.is_empty() {
            println!("Name cannot be empty. Please try again.");
            continue;
        }
        if name.len() > 20 {
            println!("Name cannot exceed 20 characters. Please choose a shorter name.");
            continue;
        }
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ') {
            println!("Name can only contain letters, numbers, and spaces.");
            continue;
        }

        // Attempt to retrieve the player by name (case-insensitive)
        match db::get_player_by_name(&conn, &name).await {
            Ok(Some(mut player)) => {
                // Existing account found: verify password if one is set
                if !player.password.trim().is_empty() {
                    let pass = ui::prompt("Enter your password: ");
                    if !verify_password(pass.trim(), &player.password) {
                        println!("Incorrect password for '{}'. Please try again.", player.name);
                        continue;
                    }
                }
                println!("\nHello, {}!", player.name);
                // Update last login timestamp to now
                player.last_login = Local::now().naive_local();
                // Launch main game menu
                game::town::main_menu(&conn, &mut player).await;
                // Save player data after gameplay
                if let Err(e) = db::update_player(&conn, &player).await {
                    println!("Failed to save player data: {}", e);
                }
                // Exit loop and keep final player data
                break player;
            }
            Ok(None) => {
                println!("No account found with the name '{}'.", name);
                let choice = ui::prompt("Would you like to create a new account? (Y/N): ");
                if choice.trim().eq_ignore_ascii_case("Y") {
                    let new_pass = ui::prompt("Enter a password (or leave blank): ");
                    let new_pass_trimmed = new_pass.trim();
                    match db::create_player(&conn, &name, new_pass_trimmed).await {
                        Ok(mut new_player) => {
                            println!("Account '{}' created successfully!", new_player.name);
                            new_player.last_login = Local::now().naive_local();
                            game::town::main_menu(&conn, &mut new_player).await;
                            if let Err(e) = db::update_player(&conn, &new_player).await {
                                println!("Failed to save new player data: {}", e);
                            }
                            break new_player;
                        }
                        Err(e) => {
                            println!("Error creating account: {:?}", e);
                        }
                    }
                } else {
                    println!("Please try again with a different name.\n");
                    continue;
                }
            }
            Err(e) => {
                println!("Error retrieving player: {}", e);
                continue;
            }
        }
    };

    // Farewell message when the player exits the game
    println!("\nThank you for playing! Goodbye, {}.", player.name);
}
