//! Core game module: defines Player struct, game constants, and shared functions (e.g., leveling up).

pub mod forest;
pub mod pvp;
pub mod romance;
pub mod town;

use sqlx::PgPool;
use chrono::NaiveDateTime;
use colored::Colorize;
use crate::db;

/// Maximum number of forest fights per player per day.
pub const MAX_DAILY_FOREST_FIGHTS: i32 = 10;

/// Player data structure representing a player's state in the game.
#[derive(Debug, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub password: String,           // upgrades to stored encrypted
    pub level: i32,
    pub exp: i32,
    pub gold: i32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub forest_fights: i32,
    pub alive: bool,
    pub romance: i32,               // romance points with Violet
    pub spouse: String,             // spouse name (e.g., "Violet" if married to NPC, or another player name)
    pub last_login: NaiveDateTime,  // ← FIX: Change from `String` to `NaiveDateTime`
}

impl Player {
    /// Calculate the experience points required for the next level.
    pub fn xp_to_next_level(&self) -> i32 {
        self.level * 100
    }
}

/// Check and perform level-ups if the player has enough experience.
/// This will increase the player's level (possibly multiple times if a lot of XP was gained)
/// and improve their stats each time.
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
