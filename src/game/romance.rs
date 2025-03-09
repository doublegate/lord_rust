//! Tavern and romance mechanics: interacting with Violet the barmaid.

use colored::Colorize;
use crate::ui::ansi_art; // Import ANSI art
use crate::game::Player;
use crate::db;
use sqlx::PgPool;

/// Flirt with Violet, the tavern barmaid. Increases romance points and possibly triggers marriage.
async fn flirt_with_violet(player: &mut Player, conn: &PgPool) {
    if player.spouse.to_lowercase() == "violet" {
        println!("Violet laughs, \"We're already married, dear!\"");
        return;
    }
    player.romance += 1;
    let responses = [
        "You wink at Violet. She smiles shyly.",
        "You compliment Violet. She giggles and blushes.",
        "You share a rose with Violet. She seems flattered.",
        "You sing a love ballad. Violet gazes at you dreamily.",
        "Overjoyed, Violet exclaims 'Yes! I will marry you!'"
    ];
    let stage = player.romance.min(responses.len() as i32) - 1;
    if stage < 0 {
        // Should not happen as romance was incremented
        println!("You try to flirt, but it feels awkward.");
        return;
    }
    println!("{}", responses[stage as usize]);
    if player.romance >= 5 {
        // Marriage triggered
        player.spouse = "Violet".to_string();
        println!("{}", "You and Violet are now married! The tavern erupts in cheers.".magenta().bold());
        let news = format!("{} has married Violet, the tavern barmaid!", player.name);
        db::log_event(conn, &news).await.ok();
    }
}

/// Visit the Dark Cloak Tavern (tavern menu).
pub async fn visit_tavern(conn: &PgPool, player: &mut Player) {
	println!("{}", ansi_art::TAVERN_SCENE); // Display ANSI Tavern Art
    println!("\nYou enter the Dark Cloak Tavern. Violet greets you with a warm smile.");
    loop {
        println!("\nTavern options:");
        println!("  1. Flirt with Violet");
        println!("  2. Buy a drink (5 gold)");
        println!("  3. Listen to gossip (read daily news)");
        println!("  4. Return to town");
        let choice = crate::ui::prompt("Choose an option: ");
        match choice.trim() {
            "1" => {
                flirt_with_violet(player, conn).await;
            }
            "2" => {
                // Buy a drink to heal
                if player.gold < 5 {
                    println!("You don't have enough gold to buy a drink.");
                } else {
                    player.gold -= 5;
                    if player.current_hp >= player.max_hp {
                        println!("You drink a refreshing ale, but you're already at full health.");
                    } else {
                        let heal = 5; // fixed small heal
                        player.current_hp += heal;
                        if player.current_hp > player.max_hp {
                            player.current_hp = player.max_hp;
                        }
                        println!("You drink a mug of ale. You feel a bit better (+{} HP).", heal);
                        println!("Current HP: {}/{}", player.current_hp, player.max_hp);
                    }
                }
            }
            "3" => {
                // Gossip = read news events
                println!("\nRecent News and Gossip:");
                match db::get_latest_events(conn, 10).await {
                    Ok(events) => {
                        if events.is_empty() {
                            println!("(No news at the moment.)");
                        } else {
                            for (date, msg) in events {
                                println!("[{}] {}", date, msg);
                            }
                        }
                    }
                    Err(err) => println!("Could not retrieve news: {}", err),
                }
                // Pause for player to read
                let _ = crate::ui::prompt("Press Enter to return to the tavern...");
            }
            "4" => {
                println!("You leave the tavern and head back to the town square.");
                break;
            }
            _ => println!("Invalid choice. Please enter 1-4."),
        }
    }
}
