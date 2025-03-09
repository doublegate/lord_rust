//! Forest exploration: encountering and fighting monsters.

use rand::Rng;
use colored::Colorize;
use crate::ui::ansi_art; // Import ANSI art
use crate::game::{Player, try_level_up};
use crate::db;
use sqlx::PgPool;

/// A simple monster representation for forest fights.
struct Monster {
    name: &'static str,
    hp: i32,
    attack: i32,
    exp_reward: i32,
    gold_reward: i32,
}

/// Generate a random monster appropriate for the player's level.
fn generate_monster(player_level: i32) -> Monster {
    // Some monster templates (name, base HP, base attack)
    static MONSTER_TEMPLATES: &[(&str, i32, i32)] = &[
        ("Wild Boar", 15, 4),
        ("Goblin", 20, 5),
        ("Ogre", 30, 6),
        ("Giant Spider", 25, 5),
        ("Black Knight", 35, 8),
        ("Forest Dragon", 50, 12),
    ];
    let mut rng = rand::rng();
    // Pick a random template
    let (name, base_hp, base_attack) = MONSTER_TEMPLATES[rng.random_range(0..MONSTER_TEMPLATES.len())];
    // Scale monster stats with player level
    let level_factor = 1 + (player_level - 1) / 2;  // moderate scaling
    let hp = base_hp * level_factor + rng.random_range(0..=5*player_level);
    let attack = base_attack * level_factor + rng.random_range(0..=player_level);
    // Determine rewards based on monster strength
    let exp_reward = hp / 2 + attack;
    let gold_reward = rng.random_range(1..=attack * 3);
    Monster {
        name,
        hp,
        attack,
        exp_reward: exp_reward.max(1),
        gold_reward: gold_reward.max(1),
    }
}
/// Enter the forest and fight monsters until the player chooses to leave or runs out of fights/HP.
pub async fn explore_forest(conn: &PgPool, player: &mut Player) {
    if player.forest_fights <= 0 {
        println!("You've exhausted your forest fights for today.");
        return;
    }
    println!("{}", ansi_art::FOREST_SCENE); // Show ANSI art before exploration starts
	println!("You venture into the forest... ({} fights left today)", player.forest_fights);
    let mut rng = rand::rng();

    // Loop for multiple fights
    while player.forest_fights > 0 && player.alive {
        // Encounter a monster
        let mut monster = generate_monster(player.level);
        println!("\nA wild {} appears! [HP: {}, Attack: {}]", monster.name, monster.hp, monster.attack);
        println!("{}", format!("Fight! {} vs {}", player.name, monster.name).yellow().bold());
        // Battle loop
        while player.alive && monster.hp > 0 {
            // Player attacks first each round
            let damage_to_monster = rng.random_range(1..=player.attack);
            monster.hp -= damage_to_monster;
            println!("You hit the {} for {} damage.", monster.name, damage_to_monster);
            if monster.hp <= 0 {
                // Monster defeated
                monster.hp = 0;
                println!("{}", format!("You have slain the {}!", monster.name).bright_green());
                // Rewards
                player.exp += monster.exp_reward;
                player.gold += monster.gold_reward;
                println!("You gain {} XP and {} gold.", monster.exp_reward, monster.gold_reward);
                // Check for level-up(s)
                try_level_up(player, conn).await;
                // Log monster kill event (optional: log only special monsters to avoid spam)
                // We'll log if monster was particularly strong (e.g., Forest Dragon or high level)
                if monster.attack > 10 {
                    let news = format!("{} defeated a {} in the forest.", player.name, monster.name);
                    if let Err(e) = db::log_event(conn, &news).await {
                        eprintln!("Failed to log monster defeat: {}", e);
                    }
                }
                break;
            }
            // Monster strikes back if still alive
            let damage_to_player = rng.random_range(1..=monster.attack);
            player.current_hp -= damage_to_player;
            println!("The {} hits you for {} damage.", monster.name, damage_to_player);
            if player.current_hp <= 0 {
                // Player dies
                player.current_hp = 0;
                player.alive = false;
                println!("{}", "You have been killed in battle...".bright_red().bold());
                let news = format!("{} was slain by a {} in the forest.", player.name, monster.name);
                db::log_event(conn, &news).await.ok();
            }
        } // end battle loop

        player.forest_fights -= 1;
        if !player.alive {
            // End exploration if player died
            break;
        }
        // If player is alive and has fights left, ask if continue
        if player.forest_fights > 0 {
            let choice = crate::ui::prompt("Fight another monster? (Y/N): ");
            if choice.trim().eq_ignore_ascii_case("N") || choice.trim().eq_ignore_ascii_case("No") {
                println!("You decide to leave the forest and head back to town.");
                break;
            }
        } else {
            println!("You have no more forest fights left today. You head back to town.");
        }
    }
}
