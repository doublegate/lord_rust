//! # Romance Module
//!
//! This module implements the tavern and romance mechanics, allowing players to
//! interact with NPCs, particularly Violet the barmaid, for social gameplay.
//!
//! ## Features
//!
//! - Tavern interactions with NPCs  
//! - Romance progression system with Violet  
//! - Marriage mechanics and benefits  
//! - Gossip and news reading  
//! - Drink purchasing for health restoration  
//!
//! ## Implementation Details
//!
//! The romance module tracks player interactions with Violet through a romance
//! counter. As this counter increases, the relationship progresses through
//! different stages, eventually leading to marriage if the player chooses.
//! Marriage provides certain gameplay benefits.

use colored::Colorize;
use crate::ui::ansi_art; // Import ANSI art
use crate::game::Player;
use crate::db;
use sqlx::PgPool;

/// Flirt with Violet, the tavern barmaid.
/// 
/// This function:
/// 1. Checks if the player is already married to Violet
/// 2. Increases romance points with each interaction
/// 3. Displays appropriate responses based on romance level
/// 4. Triggers marriage when romance reaches threshold
/// 5. Logs the marriage event in the news
/// 
/// # Arguments
/// 
/// * `player` - Mutable reference to the player's data structure
/// * `conn` - Database connection pool for logging events
async fn flirt_with_violet(player: &mut Player, conn: &PgPool) {
    // Check if already married to Violet
    if player.spouse.to_lowercase() == "violet" {
        println!("Violet laughs, \"We're already married, dear!\"");
        return;
    }
    
    // Increase romance points
    player.romance += 1;
    
    // Different responses based on romance level
    let responses = [
        "You wink at Violet. She smiles shyly.",
        "You compliment Violet. She giggles and blushes.",
        "You share a rose with Violet. She seems flattered.",
        "You sing a love ballad. Violet gazes at you dreamily.",
        "Overjoyed, Violet exclaims 'Yes! I will marry you!'"
    ];
    
    // Get appropriate response for current romance level
    let stage = player.romance.min(responses.len() as i32) - 1;
    if stage < 0 {
        // Should not happen as romance was incremented
        println!("You try to flirt, but it feels awkward.");
        return;
    }
    
    // Display the response
    println!("{}", responses[stage as usize]);
    
    // Check if romance level triggers marriage
    if player.romance >= 5 {
        // Marriage triggered
        player.spouse = "Violet".to_string();
        println!("{}", "You and Violet are now married! The tavern erupts in cheers.".magenta().bold());
        
        // Log the marriage event
        let news = format!("{} has married Violet, the tavern barmaid!", player.name);
        let news_owned = news.to_string();
        let conn = conn.clone();
        tokio::spawn(async move {
            db::log_event(&conn, &news_owned).await.ok();
        });
    }
}

/// Visit the Dark Cloak Tavern and interact with NPCs.
/// 
/// This function:
/// 1. Displays the tavern scene
/// 2. Presents a menu of tavern activities
/// 3. Processes the player's choices
/// 4. Handles flirting, drinking, and gossip interactions
/// 5. Continues until the player chooses to leave
/// 
/// # Arguments
/// 
/// * `conn` - Database connection pool for persistence operations
/// * `player` - Mutable reference to the player's data structure
pub async fn visit_tavern(conn: &PgPool, player: &mut Player) {
    // Display the tavern scene
    println!("{}", ansi_art::TAVERN_SCENE);
    println!("\nYou enter the Dark Cloak Tavern. Violet greets you with a warm smile.");
    
    // Tavern menu loop
    loop {
        // Display tavern options
        println!("\n=== Tavern Options ===");
        println!("1. Flirt with Violet");
        println!("2. Buy a drink (5 gold)");
        println!("3. Listen to gossip (read daily news)");
        println!("4. Return to town");
        
        // Get player's choice
        let choice = crate::ui::prompt("Choose an option: ");
        
        match choice.trim() {
            "1" => {
                // Flirt with Violet
                flirt_with_violet(player, conn).await;
            }
            "2" => {
                // Buy a drink to restore health
                if player.gold < 5 {
                    println!("You don't have enough gold for a drink.");
                } else {
                    player.gold -= 5;
                    
                    // Calculate health restoration (25% of max HP)
                    let heal_amount = (player.max_hp / 4).max(1);
                    let old_hp = player.current_hp;
                    player.current_hp = (player.current_hp + heal_amount).min(player.max_hp);
                    
                    // Display healing message
                    println!("You enjoy a refreshing drink. It restores {} HP.", player.current_hp - old_hp);
                    println!("Current HP: {}/{}", player.current_hp, player.max_hp);
                    
                    // Special message if married to Violet
                    if player.spouse.to_lowercase() == "violet" {
                        println!("Violet gives you a wink and a free refill!");
                        
                        // Bonus healing for married players
                        let bonus_heal = (player.max_hp / 8).max(1);
                        let old_hp = player.current_hp;
                        player.current_hp = (player.current_hp + bonus_heal).min(player.max_hp);
                        
                        println!("The extra drink restores {} more HP.", player.current_hp - old_hp);
                    }
                }
            }
            "3" => {
                // Listen to gossip (read news)
                println!("\nThe patrons share the latest rumors...");
                
                // Retrieve and display recent news
                match db::get_latest_events(conn, 5).await {
                    Ok(events) => {
                        if events.is_empty() {
                            println!("It's been quiet lately. No interesting news.");
                        } else {
                            for (date, message) in events {
                                println!("[{}] {}", date.format("%Y-%m-%d %H:%M"), message);
                            }
                        }
                    }
                    Err(e) => {
                        println!("The tavern is too noisy to hear clearly. (Error: {})", e);
                    }
                }
                
                println!("\nPress Enter to continue...");
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            "4" => {
                // Return to town
                println!("You bid farewell to Violet and leave the tavern.");
                return;
            }
            _ => {
                // Invalid choice
                println!("That's not a valid option. Please try again.");
            }
        }
    }
}

