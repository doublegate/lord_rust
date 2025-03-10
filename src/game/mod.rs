//! # Game Module
//!
//! Core game module that defines the Player struct, game constants, and shared functions.
//! This module serves as the foundation for all gameplay mechanics and player interactions.
//!
//! ## Features
//!
//! - Player data structure with stats, inventory, and state  
//! - Experience and leveling system  
//! - Combat mechanics for PvE and PvP  
//! - Social interactions and romance options  
//!
//! ## Implementation Details
//!
//! The game module is divided into several submodules:  
//! - `forest`: Handles monster encounters and combat in the forest  
//! - `pvp`: Manages player-vs-player duels and rankings  
//! - `romance`: Implements NPC and player romance options  
//! - `town`: Provides the main game menu and hub functionality  

/// Forest exploration and monster combat module
pub mod forest;
/// Player versus player combat module
pub mod pvp;
/// NPC and player romance interactions module
pub mod romance;
/// Town hub and main menu module
pub mod town;

use sqlx::PgPool;
use chrono::NaiveDateTime;
use colored::Colorize;
use crate::db;

/// Maximum number of forest fights per player per day.
/// This constant limits how many monsters a player can fight each day.
pub const MAX_DAILY_FOREST_FIGHTS: i32 = 10;

/// Player data structure representing a player's state in the game.
/// 
/// This struct contains all the information about a player, including:
/// - Basic identification (id, name)
/// - Authentication data (password)
/// - Character stats (level, hp, attack, defense)
/// - Game progress (exp, gold, forest_fights)
/// - Social status (alive, romance, spouse)
/// - Session data (last_login)
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Player {
    /// Unique player identifier
    pub id: i32,
    /// Player's character name
    pub name: String,
    /// Hashed password for authentication
    pub password: String,
    /// Current experience level
    pub level: i32,
    /// Experience points accumulated
    pub exp: i32,
    /// Gold pieces carried
    pub gold: i32,
    /// Current hit points
    pub current_hp: i32,
    /// Maximum hit points
    pub max_hp: i32,
    /// Attack power (affects damage dealt)
    pub attack: i32,
    /// Defense power (affects damage received)
    pub defense: i32,
    /// Remaining forest fights for the day
    pub forest_fights: i32,
    /// Whether the player is alive or dead
    pub alive: bool,
    /// Romance points with Violet (NPC)
    pub romance: i32,
    /// Name of spouse (if married)
    pub spouse: String,
    /// Timestamp of last login
    pub last_login: NaiveDateTime,
}

impl Player {
    /// Calculate the experience points required for the next level.
    /// 
    /// The formula is simple: level * 100
    /// This means higher levels require more experience to advance.
    /// 
    /// # Returns
    /// 
    /// The amount of experience points needed to reach the next level.
    pub fn xp_to_next_level(&self) -> i32 {
        self.level * 100
    }
}

/// Check and perform level-ups if the player has enough experience.
/// 
/// This function:
/// 1. Checks if the player has enough XP to level up
/// 2. Deducts the required XP and increases level
/// 3. Improves player stats (HP, attack, defense)
/// 4. Announces the level-up to the player
/// 5. Logs the achievement in the game news
/// 
/// Multiple level-ups can occur in a single call if the player
/// has gained enough experience.
/// 
/// # Arguments
/// 
/// * `player` - Mutable reference to the player being checked for level-up
/// * `conn` - Database connection pool for logging the event
pub async fn try_level_up(player: &mut Player, conn: &PgPool) {
    while player.exp >= player.xp_to_next_level() {
        player.exp -= player.xp_to_next_level();
        player.level += 1;
        // Increase stats upon leveling up
        player.max_hp += 10;
        player.current_hp = player.max_hp;
        player.attack += 2;
        player.defense += 1;
        println!("{}", format!("Congratulations! You are now Level {}.", player.level).bright_green().bold());
        // Log the level-up event in the news
        let news = format!("{} has reached Level {}!", player.name, player.level);
        let news_owned = news.to_string();
        let conn = conn.clone();
        tokio::spawn(async move {
            db::log_event(&conn, &news_owned).await.ok();
        });
    }
}
