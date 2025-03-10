//! Player vs Player combat: choosing an opponent and simulating a duel.
use rand::Rng;
use colored::Colorize;
use crate::ui::ansi_art; // Import ANSI art
use crate::game::Player;
use crate::db;
use sqlx::PgPool;

pub async fn challenge_player(conn: &PgPool, player: &mut Player) {
    println!("{}", ansi_art::DUEL_SCENE.replace("{player_name}", &player.name)
                                       .replace("{opponent_name}", "Unknown Opponent"));
    // List potential opponents (alive players other than the current player)
    let opponents = match db::list_alive_players(conn, player.id).await {
        Ok(list) => list,
        Err(e) => {
            println!("Error fetching player list: {}", e);
            return;
        }
    };
    if opponents.is_empty() {
        println!("No other heroes are currently available to duel.");
        return;
    }
    println!("\nWho would you like to challenge?");
    for (idx, opp) in opponents.iter().enumerate() {
        println!("  {}. {} (Level {})", idx + 1, opp.name, opp.level);
    }
    println!("  0. Nevermind (cancel)");
    let input = crate::ui::prompt("Enter the number of the player to fight: ");
    let choice = input.trim().parse::<usize>();
    let index = match choice {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice.");
            return;
        }
    };
    if index == 0 {
        println!("Challenge canceled. You return to town.");
        return;
    }
    if index < 1 || index > opponents.len() {
        println!("Invalid selection.");
        return;
    }
    let target_info = &opponents[index - 1];
    // Load full target player data
    let mut target = match db::get_player_by_id(conn, target_info.id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            println!("Could not find that player.");
            return;
        }
        Err(e) => {
            println!("Error retrieving player: {}", e);
            return;
        }
    };
    if !target.alive {
        println!("{} is not available to fight.", target.name);
        return;
    }
    println!("\nYou challenge {} to a duel!", target.name);
    println!("{} draws their weapon...", target.name);
    // Simulate the duel
    let mut rng = rand::rng();
    while player.alive && target.alive {
        // Player (attacker) strikes first
        let dmg_to_target = rng.random_range(1..=player.attack);
        target.current_hp -= dmg_to_target;
        println!("You hit {} for {} damage.", target.name, dmg_to_target);
        if target.current_hp <= 0 {
            target.alive = false;
            target.current_hp = 0;
            println!("{}", format!("You have defeated {}!", target.name).bright_green().bold());
            // Loot and reward
            let stolen_gold = target.gold / 2;
            if stolen_gold > 0 {
                target.gold -= stolen_gold;
                player.gold += stolen_gold;
                println!("You loot {} gold from {}.", stolen_gold, target.name);
            }
            let xp_gain = target.level * 50;
            if xp_gain > 0 {
                player.exp += xp_gain;
                println!("You gain {} experience from the victory!", xp_gain);
                crate::game::try_level_up(player, conn).await;
            }
            // Log PvP victory
            let news = format!("{} defeated {} in a duel!", player.name, target.name);
            if let Err(e) = db::log_event(conn, &news).await {
                eprintln!("Failed to log event: {}", e);
            }
            break;
        }
        // Opponent strikes back if still alive
        let dmg_to_player = rng.random_range(1..=target.attack);
        player.current_hp -= dmg_to_player;
        println!("{} hits you for {} damage.", target.name, dmg_to_player);
        if player.current_hp <= 0 {
            player.alive = false;
            player.current_hp = 0;
            println!("{}", "You have been defeated in combat...".bright_red().bold());
            // When player dies in PvP, opponent might loot
            let stolen_gold = player.gold / 2;
            if stolen_gold > 0 {
                player.gold -= stolen_gold;
                target.gold += stolen_gold;
            }
            // Log PvP loss
            let news = format!("{} was killed by {} in a duel!", player.name, target.name);
            if let Err(e) = db::log_event(conn, &news).await {
                eprintln!("Failed to log event: {}", e);
            }
            break;
        }
        // Loop continues until one is defeated
    }

    // Update both players in the database after the duel
    if let Err(e) = (async {
        let mut tx = conn.begin().await?;
        sqlx::query(
            r#"UPDATE players SET
                level=$1, exp=$2, gold=$3, current_hp=$4, max_hp=$5,
                attack=$6, defense=$7, forest_fights=$8, alive=$9,
                romance=$10, spouse=$11, last_login=NOW()
            WHERE id=$12"#
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
        .execute(&mut *tx).await?;
        sqlx::query(
            r#"UPDATE players SET
                level=$1, exp=$2, gold=$3, current_hp=$4, max_hp=$5,
                attack=$6, defense=$7, forest_fights=$8, alive=$9,
                romance=$10, spouse=$11, last_login=NOW()
            WHERE id=$12"#
        )
        .bind(target.level)
        .bind(target.exp)
        .bind(target.gold)
        .bind(target.current_hp)
        .bind(target.max_hp)
        .bind(target.attack)
        .bind(target.defense)
        .bind(target.forest_fights)
        .bind(target.alive)
        .bind(target.romance)
        .bind(&target.spouse)
        .bind(target.id)
        .execute(&mut *tx).await?;
        tx.commit().await?;
        Ok::<(), sqlx::Error>(())
    }).await {
        println!("Failed to save duel results: {}", e);
    }
    if !player.alive {
        // If the current player died, they can't continue acting this day
        println!("You limp back to town as a spirit, awaiting tomorrow for another chance...");
    } else {
        println!("The duel is over. You and {} return to town.", target.name);
    }
    // Pause to allow player to see the outcome
    let _ = crate::ui::prompt("Press Enter to continue...");
}
