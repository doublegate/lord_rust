//! Town menu: presents options to the player and calls the appropriate game functions.

use colored::Colorize;
use crate::ui::ansi_art; // Import ANSI art
use crate::game::{Player, forest, pvp, romance};
use crate::db;
use sqlx::PgPool;

pub async fn main_menu(conn: &PgPool, player: &mut Player) {
    // Main loop for the town (runs until player quits or dies)
	println!("{}", ansi_art::TOWN_SQUARE);
    while player.alive {
        // Display player status summary at the top of each loop
        println!("\n==================================================");
        println!(
            "{} (Level {})  HP: {}/{}  Exp: {}/{}  Gold: {}",
            player.name,
            player.level,
            player.current_hp,
            player.max_hp,
            player.exp,
            player.xp_to_next_level(),
            player.gold
        );
        if !player.spouse.is_empty() {
            // Show spouse status if married
            let spouse = if player.spouse.to_lowercase() == "violet" {
                "Violet (your wife)".to_string()
            } else {
                format!("{} (spouse)", player.spouse)
            };
            println!("Spouse: {}", spouse);
        }
        println!("==================================================");
        // Display town menu options with some color for emphasis
        println!("{}", "1. Enter the Forest".green());
        println!("{}", "2. Visit the Tavern".magenta());
        println!("{}", "3. Duel another player".red());
        println!("{}", "4. View your character".blue());
        println!("{}", "5. Read Daily News".cyan());
        println!("{}", "6. Leaderboard".yellow());
        println!("7. Save and Quit");

        let choice = crate::ui::prompt("What would you like to do? ");
        match choice.trim() {
            "1" => {
                forest::explore_forest(conn, player).await;
                if !player.alive {
                    // Player died in the forest
                    break;
                }
            }
            "2" => {
                romance::visit_tavern(conn, player).await;
                // Visiting tavern should not kill the player, so nothing special to check
            }
            "3" => {
                pvp::challenge_player(conn, player).await;
                if !player.alive {
                    // Player died in a duel
                    break;
                }
            }
            "4" => {
                // View character details
                println!("\nCharacter Information:");
                println!("Name: {}", player.name);
                println!("Level: {} (Exp: {}/{})", player.level, player.exp, player.xp_to_next_level());
                println!("Health: {}/{}", player.current_hp, player.max_hp);
                println!("Attack: {}  Defense: {}", player.attack, player.defense);
                println!("Gold: {}", player.gold);
                if !player.spouse.is_empty() {
                    println!("Spouse: {}", player.spouse);
                } else {
                    println!("Spouse: (none)");
                }
                // Romance points and daily fights (for debug/interest)
                println!("Romance points (with Violet): {}", player.romance);
                println!("Forest fights remaining today: {}", player.forest_fights);
                let _ = crate::ui::prompt("Press Enter to continue...");
            }
            "5" => {
                // Read daily news log
                println!("\nDaily News Bulletin:");
                match db::get_latest_events(conn, 10).await {
                    Ok(events) => {
                        if events.is_empty() {
                            println!("No news yet today.");
                        } else {
                            for (date, msg) in events {
                                println!("[{}] {}", date, msg);
                            }
                        }
                    }
                    Err(err) => println!("Error loading news: {}", err),
                }
                let _ = crate::ui::prompt("Press Enter to return to town...");
            }
            "6" => {
                // Show top players leaderboard
                println!("\nHall of Fame - Top Heroes:");
                match db::get_top_players(conn, 10).await {
                    Ok(list) => {
                        if list.is_empty() {
                            println!("No players to display.");
                        } else {
                            for (rank, info) in list.iter().enumerate() {
                                println!("{}. {} - Level {}", rank + 1, info.name, info.level);
                            }
                        }
                    }
                    Err(err) => println!("Could not retrieve leaderboard: {}", err),
                }
                let _ = crate::ui::prompt("Press Enter to continue...");
            }
            "7" => {
                println!("Saving your progress...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number from 1 to 7.");
            }
        }
    } // end while

    if !player.alive {
        println!("\n*** You have perished for today. ***");
        println!("Rest well, hero. Tomorrow is a new day.");
    }
}
