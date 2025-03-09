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

/// Database module: Handles all PostgreSQL interactions for player data and game state
mod db;
/// Game module: Contains all game mechanics, combat systems, and area-specific logic
mod game;
/// UI module: Provides terminal interface utilities, ANSI color support, and input handling
mod ui;

use crate::db::verify_password;
use chrono::Local;
use tokio;

#[tokio::main]
async fn main() {
    // Clear the terminal screen for a clean start
    // This ensures the game UI begins with a fresh display
    ui::clear_screen();

    // Initialize the PostgreSQL database connection pool and apply schema if needed
    // This establishes the connection to the database and ensures all required tables exist
    // The connection pool is used throughout the application for all database operations
    let conn = db::init_db_pool()
        .await
        .expect("Failed to initialize database");

    // Perform daily reset operations if a new day has started since last reset
    // This includes:
    // - Resetting player forest fights to the maximum daily allowance
    // - Reviving any dead players (setting alive=true)
    // - Restoring player health to maximum
    // - Updating the last_reset date in the game_state table
    db::daily_reset(&conn)
        .await
        .expect("Failed to perform daily reset");

    // Display the game title with ANSI art for visual appeal
    ui::show_title();
    println!("*** Welcome to \x1B[1mLegend of the Red Dragon\x1B[0m (Rust Edition)! ***");
    println!("  By DoubleGate -+-+-+- ver.0.2.0 -+-+-+- March 8th, 2025");
    println!(); // blank line for better readability

    // User authentication loop: continues until a valid login occurs (either an existing user logs in
    // or a new account is created and automatically logged in)
    let player = loop {
        // Prompt for player name with a simple text input
        // NOTE: We'll trim whitespace and optionally lowercase for consistency
        let name_input = ui::prompt("Enter your name: ");
        // Trim the user input to remove any trailing newline/spaces
        let name = name_input.trim().to_string();

        // Validate that the name is not empty
        if name.is_empty() {
            println!("Name cannot be empty. Please try again.");
            continue;
        }

        // Attempt to retrieve the player by name (case-insensitive)
        // The database query uses LOWER(name) in db::get_player_by_name
        match db::get_player_by_name(&conn, &name).await {
            Some(mut player) => {
                // An existing account was found: perform password verification if password is set
                // Passwords are optional, so only verify if the player has set one
                if !player.password.trim().is_empty() {
                    let pass = ui::prompt("Enter your password: ");
                    // Instead of directly comparing strings, use our Argon2 verification helper:
                    if !verify_password(pass.trim(), &player.password) {
                        println!("Incorrect password for '{}'. Please try again.", player.name);
                        continue;
                    }
                }

                // Successful login greeting
                println!("\nHello, {}!", player.name);

                // Update last login timestamp with the current date and time
                player.last_login = Local::now().naive_local(); // ✅ Correct NaiveDateTime type

                // Launch the main game menu and gameplay loop
                game::town::main_menu(&conn, &mut player).await;

                // Save player data after gameplay session
                // This ensures all progress, stats, and inventory changes are persisted
                db::update_player(&conn, &player)
                    .await
                    .expect("Failed to save player data");

                // Exit the login loop with the final player data
                break player;
            }
            None => {
                // No player found with the given name
                println!("No account found with the name '{}'.", name);

                // Offer to create a new account with this name
                let choice = ui::prompt("Would you like to create a new account? (Y/N): ");
                if choice.trim().eq_ignore_ascii_case("Y") {
                    // User wants to create a new account
                    let new_pass = ui::prompt("Enter a password (or leave blank): ");
                    let new_pass_trimmed = new_pass.trim();

                    // Attempt to create the player in the database
                    match db::create_player(&conn, &name, new_pass_trimmed).await {
                        Ok(mut new_player) => {  // Now directly receives the new player
                            println!("Account '{}' created successfully!", new_player.name);
                            new_player.last_login = Local::now().naive_local(); // ✅ Correct
                            game::town::main_menu(&conn, &mut new_player).await;
                            db::update_player(&conn, &new_player)
                                .await
                                .expect("Failed to save new player data");
                            break new_player;
                        }
                        Err(e) => {
                            println!("Error creating account: {:?}", e);
                        }
                    }
                } else {
                    // User declined to create a new account
                    println!("Please try again with a different name.\n");
                    continue;
                }
            }
        }
    };

    // Farewell message when the player exits the game
    println!("\nThank you for playing! Goodbye, {}.", player.name);
}
