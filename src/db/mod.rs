//! # Database Module
//!
//! This module provides a PostgreSQL-backed persistence layer for the Legend of the Red Dragon game.
//! It handles all database operations including player management, game state tracking, and event logging.
//!
//! ## Features
//!
//! - Connection pooling with configurable connection limits  
//! - Secure password storage using Argon2 hashing algorithm  
//! - Daily game state reset mechanism  
//! - Player data persistence and retrieval  
//! - Game event logging and history  
//! - Player rankings and statistics  
//!
//! ## Technical Implementation
//!
//! - Uses SQLx for type-safe asynchronous database operations  
//! - Implements Argon2 password hashing for secure authentication  
//! - Provides transaction support for multi-step operations  
//! - Uses prepared statements to prevent SQL injection  
//! - Implements connection pooling for efficient database access  
//!
//! ## Database Schema
//!
//! The database consists of three main tables:  
//! - `players`: Stores player data including stats, inventory, and authentication  
//! - `news`: Records game events and player achievements  
//! - `game_state`: Maintains global game state including daily reset tracking  
//! 
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::game::Player;
use chrono::{Local, NaiveDateTime};
use dotenvy::dotenv;
use std::env;
// Password hashing dependencies
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng, PasswordHasher},
};

/// Player information structure for leaderboards and player listings.
/// 
/// This is a lightweight version of the full Player struct, containing
/// only the information needed for display in lists and rankings.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct PlayerInfo {
    /// Unique player identifier
    pub id: i32,
    /// Player's character name
    pub name: String,
    /// Player's experience level
    pub level: i32,
}

/// Initialize the PostgreSQL connection pool and set up the database schema.
/// 
/// This function:  
/// 1. Loads environment variables from .env file  
/// 2. Establishes a connection pool to the PostgreSQL database  
/// 3. Creates the necessary tables if they don't exist  
/// 4. Initializes the game state with default values  
/// 
/// # Returns
/// 
/// A connection pool that can be used for database operations throughout the application.
/// 
/// # Errors
/// 
/// Returns a `sqlx::Error` if:  
/// - The `DATABASE_URL` environment variable is not set  
/// - The connection to the database fails  
/// - Any of the schema initialization queries fail  
pub async fn init_db_pool() -> Result<PgPool, sqlx::Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => return Err(sqlx::Error::Configuration(Box::new(e))),
    };

    // Create a connection pool with a maximum of 10 connections
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    // Create the players table if it doesn't exist
    // This table stores all player data including authentication, stats, and game progress
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id SERIAL PRIMARY KEY,
            name TEXT UNIQUE,
            password TEXT,
            level INTEGER,
            exp INTEGER,
            gold INTEGER,
            current_hp INTEGER,
            max_hp INTEGER,
            attack INTEGER,
            defense INTEGER,
            forest_fights INTEGER,
            alive BOOLEAN,
            romance INTEGER,
            spouse TEXT,
            last_login TIMESTAMP
        )
        "#).execute(&pool).await?;

    // Create the news table if it doesn't exist
    // This table stores game events and announcements
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS news (
            id SERIAL PRIMARY KEY,
            date TIMESTAMP,
            message TEXT
        )
        "#).execute(&pool).await?;

    // Create the game_state table if it doesn't exist
    // This table stores global game state variables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS game_state (
            key TEXT PRIMARY KEY,
            value TEXT
        )
        "#).execute(&pool).await?;

    // Create indexes to optimize queries for player lookup and leaderboard
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_name_lower ON players (LOWER(name))")
        .execute(&pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_level_exp ON players (level DESC, exp DESC)")
        .execute(&pool).await?;

    // Initialize the last_reset value if it doesn't exist
    // This is used to track when the daily reset was last performed
    let today = Local::now().format("%Y-%m-%d").to_string();
    sqlx::query("INSERT INTO game_state (key, value) VALUES ('last_reset', $1) ON CONFLICT (key) DO NOTHING")
        .bind(today)
        .execute(&pool).await?;

    Ok(pool)
}

/// Perform daily game reset operations if a new day has started.
/// 
/// This function:  
/// 1. Checks if the current date is different from the last reset date  
/// 2. If it is, resets player forest fights, revives dead players, and restores health  
/// 3. Logs the reset event to the news table  
/// 4. Updates the last_reset date in the game_state table  
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool
/// 
/// # Returns
/// 
/// `Ok(())` if the reset was successful or not needed, or a `sqlx::Error` if any database operation fails.
pub async fn daily_reset(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Get the current date in YYYY-MM-DD format
    let today = Local::now().format("%Y-%m-%d").to_string();

    // Retrieve the date of the last reset from the database, using a transaction for consistency
    let mut tx = pool.begin().await?;
    let last_reset: Option<String> = sqlx::query_scalar("SELECT value FROM game_state WHERE key = 'last_reset'")
        .fetch_optional(&mut *tx).await?;
    // If the last reset was before today (or missing), perform the reset
    if last_reset.as_deref().unwrap_or("") < today.as_str() {
        // Reset player forest fights, revive dead players, and restore health
        sqlx::query("UPDATE players SET forest_fights = $1, alive = TRUE, current_hp = max_hp")
            .bind(crate::game::MAX_DAILY_FOREST_FIGHTS as i32)
            .execute(&mut *tx).await?;

        // Log the reset event to the news table
        let reset_message = "A new day dawns in the realm. All heroes feel refreshed.";
        sqlx::query("INSERT INTO news (date, message) VALUES (NOW(), $1)")
            .bind(reset_message)
            .execute(&mut *tx).await?;

        // Update the last_reset date in the game_state table
        sqlx::query("INSERT INTO game_state (key, value) VALUES ('last_reset', $1) ON CONFLICT (key) DO UPDATE SET value = $1")
            .bind(today)
            .execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Create a new player with secure password storage.
/// 
/// This function:  
/// 1. Normalizes the player name by trimming whitspace  
/// 2. Securely hashes the password using Argon2 (if provided)  
/// 3. Creates a new player record with default starting values  
/// 4. Immediately returns the created player (ensures it exists in DB)  
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `name` - The player's chosen name  
/// * `password` - The player's password (optional, can be empty)  
/// 
/// # Returns
/// 
/// `Ok(Player)` if the player was created successfully, or a `sqlx::Error` if:  
/// - The player name already exists (unique constraint violation)  
/// - Any other database operation fails  
/// 
/// # Security
/// 
/// Passwords are hashed using the Argon2 algorithm with a random salt.  
/// Empty passwords are stored as empty strings to indicate no password is required.
use sqlx::Row; // Needed for row.get()
pub async fn create_player(pool: &PgPool, name: &str, password: &str) -> Result<Player, sqlx::Error> {
    let normalized_name = name.trim(); // Force whitespace trim for consistency

    let hashed_password = if password.trim().is_empty() {
        "".to_string()
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(password.trim().as_bytes(), &salt) {
            Ok(ph) => ph,
            Err(e) => return Err(sqlx::Error::Protocol(e.to_string().into())),
        };
        password_hash.to_string()
    };

    let row = sqlx::query(
        r#"
        INSERT INTO players
        (name, password, level, exp, gold, current_hp, max_hp, attack, defense,
         forest_fights, alive, romance, spouse, last_login)
        VALUES ($1, $2, 1, 0, 100, 20, 20, 5, 2, $3, TRUE, 0, '', NOW())
        RETURNING *;
        "#,
    )
    .bind(normalized_name)
    .bind(hashed_password)
    .bind(crate::game::MAX_DAILY_FOREST_FIGHTS as i32)
    .fetch_one(pool) // Fetch the inserted row
    .await?;

    // Map row into `Player` struct manually
    let player = Player {
        id: row.get("id"),
        name: row.get("name"),
        password: row.get("password"),
        level: row.get("level"),
        exp: row.get("exp"),
        gold: row.get("gold"),
        current_hp: row.get("current_hp"),
        max_hp: row.get("max_hp"),
        attack: row.get("attack"),
        defense: row.get("defense"),
        forest_fights: row.get("forest_fights"),
        alive: row.get("alive"),
        romance: row.get("romance"),
        spouse: row.get("spouse"),
        last_login: row.try_get::<NaiveDateTime, _>("last_login")?, 
    };

    Ok(player)
}

/// Retrieve a player by name using case-insensitive matching.
/// 
/// This function:  
/// 1. Trims the input name to normalize it  
/// 2. Performs a case-insensitive database lookup  
/// 3. Returns the full Player struct if found  
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `name` - The player name to search for  
/// 
/// # Returns
/// 
/// `Ok(Some(Player))` if a player with the given name exists, `Ok(None)` if no match is found, or a `sqlx::Error` if the query fails.
/// 
/// # Security Note
/// 
/// The returned Player struct contains the hashed password, not the plaintext password.  
/// Use the `verify_password` function to check if a provided password matches the hash.
pub async fn get_player_by_name(pool: &PgPool, name: &str) -> Result<Option<Player>, sqlx::Error> {
    // Normalize the input name by trimming whitespace
    let trimmed_name = name.trim(); // Consistency in retrieval

    // Perform a case-insensitive lookup using LOWER() in SQL
    let player_opt = sqlx::query_as::<_, Player>(
        r#"SELECT * FROM players WHERE LOWER(name) = LOWER($1)"#
    )
        .bind(trimmed_name)
        .fetch_optional(pool)
        .await?;
    Ok(player_opt)
}

/// Retrieve a player by their unique ID.
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `player_id` - The unique player ID to search for  
/// 
/// # Returns
/// 
/// `Ok(Some(Player))` if a player with the given ID exists, `Ok(None)` if no match is found, or a `sqlx::Error` if the query fails.
pub async fn get_player_by_id(pool: &PgPool, player_id: i32) -> Result<Option<Player>, sqlx::Error> {
    let player_opt = sqlx::query_as::<_, Player>(
        r#"SELECT * FROM players WHERE id = $1"#
    )
        .bind(player_id)
        .fetch_optional(pool)
        .await?;
    Ok(player_opt)
}

/// Update a player's data in the database.
/// 
/// This function:  
/// 1. Updates all player fields except the password  
/// 2. Sets the last_login timestamp to the current time  
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `player` - The Player struct containing the updated data  
/// 
/// # Returns
/// 
/// `Ok(())` if the update was successful, or a `sqlx::Error` if the database operation fails.
/// 
/// # Note
/// 
/// This function does not update the password field. To change a password, use a dedicated function that properly hashes the new password.
pub async fn update_player(pool: &PgPool, player: &Player) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE players SET
            level=$1, exp=$2, gold=$3, current_hp=$4, max_hp=$5,
            attack=$6, defense=$7, forest_fights=$8, alive=$9,
            romance=$10, spouse=$11, last_login=NOW()
        WHERE id=$12
        "#,
    )
    .bind(player.level)
    .bind(player.exp)
    .bind(player.gold)
    .bind(player.current_hp)
    .bind(player.max_hp)
    .bind(player.attack)
    .bind(player.defense)
    .bind(player.forest_fights)
    .bind(player.alive)
    .bind(player.romance)
    .bind(&player.spouse)
    .bind(player.id)
    .execute(pool).await?;

    Ok(())
}

/// Log a game event to the news table.
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `message` - The event message to log  
/// 
/// # Returns
/// 
/// `Ok(())` if the event was logged successfully, or a `sqlx::Error` if the database operation fails.
pub async fn log_event(pool: &PgPool, message: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO news (date, message) VALUES (NOW(), $1)")
        .bind(message)
        .execute(pool).await?;
    Ok(())
}

/// Retrieve recent game events from the news table.
/// 
/// This function:  
/// 1. Retrieves the most recent events up to the specified limit  
/// 2. Returns them in chronological order (oldest first)  
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `limit` - The maximum number of events to retrieve  
/// 
/// # Returns
/// 
/// A vector of tuples containing the event date and message, or a `sqlx::Error` if the database operation fails.
pub async fn get_latest_events(pool: &PgPool, limit: i64) -> Result<Vec<(NaiveDateTime, String)>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct NewsEvent {
        date: NaiveDateTime,
        message: String,
    }

    // Retrieve the events in reverse chronological order (newest first)
    let events = sqlx::query_as::<_, NewsEvent>(
        "SELECT date, message FROM news ORDER BY id DESC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    // Reverse the order to get chronological order (oldest first) and convert to tuples
    Ok(events.into_iter().map(|e| (e.date, e.message)).rev().collect())
}

/// List all alive players, excluding the specified player.
/// 
/// This function is typically used to show potential PvP targets.
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `exclude_id` - The ID of the player to exclude from the results  
/// 
/// # Returns
/// 
/// A vector of PlayerInfo structs for all alive players except the excluded one, or a `sqlx::Error` if the database operation fails.
pub async fn list_alive_players(pool: &PgPool, exclude_id: i32) -> Result<Vec<PlayerInfo>, sqlx::Error> {
    sqlx::query_as::<_, PlayerInfo>(
        "SELECT id, name, level FROM players WHERE alive = true AND id != $1 ORDER BY name"
    )
    .bind(exclude_id)
    .fetch_all(pool)
    .await
}

/// Retrieve the top players ranked by level and experience.
/// 
/// # Parameters
/// 
/// * `pool` - The database connection pool  
/// * `limit` - The maximum number of players to retrieve  
/// 
/// # Returns
/// 
/// A vector of PlayerInfo structs for the top players, or a `sqlx::Error` if the database operation fails.
pub async fn get_top_players(pool: &PgPool, limit: i64) -> Result<Vec<PlayerInfo>, sqlx::Error> {
    sqlx::query_as::<_, PlayerInfo>(
        "SELECT id, name, level FROM players ORDER BY level DESC, exp DESC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Verify a password against a stored hash using Argon2.
/// 
/// This function:  
/// 1. Handles the special case of empty passwords  
/// 2. Parses the stored hash  
/// 3. Verifies the entered password against the hash using Argon2  
/// 
/// # Parameters
/// 
/// * `entered_pw` - The password entered by the user  
/// * `hashed` - The stored password hash from the database  
/// 
/// # Returns
/// 
/// `true` if the password matches, `false` otherwise.
/// 
/// # Security
/// 
/// This function uses the Argon2 algorithm for password verification, which is resistant to brute force attacks and timing attacks.
pub fn verify_password(entered_pw: &str, hashed: &str) -> bool {
    // If the stored 'hashed' is empty, that implies no password is set.
    if hashed.trim().is_empty() {
        // Then the user only passes if they entered no password.
        return entered_pw.trim().is_empty();
    }

    // Use the default Argon2 configuration
    let argon2 = Argon2::default();

    // Parse the stored hash
    match PasswordHash::new(hashed) {
        Ok(parsed_hash) => {
            // Verify the entered password against the parsed hash
            argon2.verify_password(entered_pw.as_bytes(), &parsed_hash).is_ok()
        }
        Err(_) => false,
    }
}
