//! # ANSI Art Module
//!
//! This module contains ASCII art with ANSI color codes for various game scenes and UI elements.
//! These art assets are used throughout the game to enhance the visual experience of the
//! text-based interface.
//!
//! ## Implementation Details
//!
//! - Uses ANSI escape sequences for colors (\x1B[XXm)
//! - Contains ASCII art for all major game areas and scenes
//! - Art is stored as string constants for easy inclusion in the UI

/// Title screen banner (ASCII art with ANSI colors) to display on game start.
/// 
/// This banner is shown when the game first launches, providing a colorful
/// welcome screen with the game title and basic menu options.
pub const TITLE_BANNER: &str = "
\x1B[31m============================================\x1B[0m
\x1B[31m<<<|*|>>> \x1B[1;31mLegend of the Red Dragon\x1B[0m \x1B[31m<<<|*|>>>\x1B[0m
\x1B[31m============================================\x1B[0m
\x1B[33mYou enter a world of monsters, heroes, and romance...\x1B[0m
\x1B[32m(1) Enter the Forest   (2) Visit the Tavern  (3) Duel Another Player\x1B[0m
\x1B[36m(4) View Your Character (5) Read Daily News  (6) Leaderboard\x1B[0m
\x1B[31m(7) Save and Quit\x1B[0m
";

/// ANSI Forest Scene for exploration.
/// 
/// This ASCII art depicts a forest scene with trees and is displayed
/// when the player enters the forest area for monster encounters.
pub const FOREST_SCENE: &str = "
\x1B[32m      ,@@@@@@@,
      ,@@@@@@/@@,  .oo8888o.
     ,@@@\\@@@/@@@,8888888888o
     @@@@\\@@@/@@@@88888888888
     @@@@/  \\@@@@@'8888888888
     '@@@@  @@@@@'  `88888888o
      '@@@@ @@@@@'    `8888888o
       '@@@@@@@@'       `8888888
       .o@@@@@@@@@o.      `888888
      .@@@@@''''@@@@@.     `8888'
     .@@@@'       '@@@.     `888
     @@'            '@@      `8'
";

/// ANSI Tavern Scene for social interactions.
/// 
/// This ASCII art depicts a tavern interior and is displayed
/// when the player visits the tavern for social interactions.
pub const TAVERN_SCENE: &str = "
\x1B[33m+----------------------------------------+
|      \x1B[31mDark Cloak Tavern\x1B[33m                 |
+----------------------------------------+
| \x1B[32mBartender\x1B[33m: \"Welcome traveler!\"         |
| \x1B[36m(1)\x1B[33m Flirt with Violet the Barmaid      |
| \x1B[36m(2)\x1B[33m Buy a drink (5 gold)               |
| \x1B[36m(3)\x1B[33m Listen to gossip (read news)       |
| \x1B[36m(4)\x1B[33m Return to Town Square              |
+----------------------------------------+\x1B[0m
";

/// ANSI Town Square for the main game hub.
/// 
/// This ASCII art depicts the town square and is displayed
/// when the player is at the main menu/town hub.
pub const TOWN_SQUARE: &str = "
\x1B[33m+========================================+
|  \x1B[31mWelcome to the Town Square\x1B[33m            |
+========================================+
| \x1B[36m(1)\x1B[33m Enter the Forest to hunt monsters  |
| \x1B[36m(2)\x1B[33m Visit the Tavern for drinks & talk |
| \x1B[36m(3)\x1B[33m Challenge another player to a duel |
| \x1B[36m(4)\x1B[33m View your stats & progress         |
| \x1B[36m(5)\x1B[33m Read the Daily News log            |
| \x1B[36m(6)\x1B[33m View the leaderboard               |
| \x1B[36m(7)\x1B[33m Save and exit                      |
+========================================+
";

/// ANSI Red Dragon for the final boss encounter.
/// 
/// This ASCII art depicts the legendary Red Dragon and is displayed
/// during the final boss encounter in the game.
pub const RED_DRAGON: &str = "
\x1B[31m      (  )   /\\   _                 (
     \\ |  (  | (  | |   _          )
   (  \\ )  )  )  )_) | |  )     (  (
  ( (\\_    (  / _)(_) | | (_)    ) )
 ( ( )|\\)  | |/  (_)  ) ) /|  (  /
 (  \\ \\) \\ | (     ) ( ( |  | ( |
  \\ ( )  )| |    /   )| (  |  ) |
   \\ ( ) ( | |   \\  /  (  (  | |
    \\ (_)  \\_/    \\/   \\_/   \\|
";

/// ANSI Duel Scene for player versus player combat.
/// 
/// This ASCII art depicts two duelists and is displayed
/// when players engage in PvP combat.
pub const DUEL_SCENE: &str = "
\x1B[31m+========================================+
|      \x1B[1;31m⚔ DUEL IN THE ARENA ⚔\x1B[0;31m             |
+========================================+
| \x1B[33mChallenger:\x1B[0m {player_name}                 |
| \x1B[33mOpponent:\x1B[0m {opponent_name}             |
| \x1B[31mFIGHT!\x1B[0m                                 |
| [💥] {player_name} attacks!               |
| [💥] {opponent_name} strikes back!    |
+========================================+
";

/// ANSI Game Over screen for player death.
/// 
/// This ASCII art is displayed when the player dies in combat,
/// indicating game over until the next daily reset.
pub const GAME_OVER: &str = "
\x1B[31m+========================================+
|         \x1B[1;31m💀 YOU HAVE DIED 💀\x1B[0;31m        |
+========================================+
| You have fallen in battle...           |
| Rest well, warrior.                     |
| Tomorrow is a new day.                  |
|                                         |
| \x1B[36mPress any key to return to town...\x1B[0m  |
+========================================+
";
