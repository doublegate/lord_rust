//! ANSI art and colored text banners for the game (e.g., title screen).

/// Title screen banner (ASCII art with ANSI colors) to display on game start.
/// ANSI title screen for LORD
pub const TITLE_BANNER: &str = "
\x1B[31m==================================================\x1B[0m
\x1B[31m<<<|*|>>> \x1B[1;31mLegend of the Red Dragon\x1B[0m \x1B[31m<<<|*|>>>\x1B[0m
\x1B[31m==================================================\x1B[0m
\x1B[33mYou enter a world of monsters, heroes, and romance...\x1B[0m
\x1B[32m(1) Enter the Forest   (2) Visit the Tavern  (3) Duel Another Player\x1B[0m
\x1B[36m(4) View Your Character (5) Read Daily News  (6) Leaderboard\x1B[0m
\x1B[31m(7) Save and Quit\x1B[0m
";

/// ANSI Forest Scene for exploration
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

/// ANSI Tavern Scene for the Dark Cloak Tavern
pub const TAVERN_SCENE: &str = "
\x1B[33m+----------------------------------------+
|      \x1B[31mDark Cloak Tavern\x1B[33m              |
+----------------------------------------+
| \x1B[32mBartender\x1B[33m: \"Welcome traveler!\"       |
| \x1B[36m(1)\x1B[33m Flirt with Violet the Barmaid    |
| \x1B[36m(2)\x1B[33m Buy a drink (5 gold)            |
| \x1B[36m(3)\x1B[33m Listen to gossip (read news)    |
| \x1B[36m(4)\x1B[33m Return to Town Square           |
+----------------------------------------+
";

/// ANSI Town Square Scene
pub const TOWN_SQUARE: &str = "
\x1B[33m+========================================+
|  \x1B[31mWelcome to the Town Square\x1B[33m            |
+========================================+
| \x1B[36m(1)\x1B[33m Enter the Forest to hunt monsters |
| \x1B[36m(2)\x1B[33m Visit the Tavern for drinks & talk|
| \x1B[36m(3)\x1B[33m Challenge another player to a duel |
| \x1B[36m(4)\x1B[33m View your stats & progress       |
| \x1B[36m(5)\x1B[33m Read the Daily News log         |
| \x1B[36m(6)\x1B[33m View the leaderboard            |
| \x1B[36m(7)\x1B[33m Save and exit                   |
+========================================+
";

/// ANSI Final Boss Scene: The Red Dragon
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

/// ANSI Duel Scene for Player vs Player combat
pub const DUEL_SCENE: &str = "
\x1B[31m+========================================+
|      \x1B[1;31m⚔ DUEL IN THE ARENA ⚔\x1B[0;31m          |
+========================================+
| \x1B[33mChallenger:\x1B[0m {player_name}              |
| \x1B[33mOpponent:\x1B[0m {opponent_name}              |
| \x1B[31mFIGHT!\x1B[0m                               |
| [💥] {player_name} attacks!          |
| [💥] {opponent_name} strikes back!   |
+========================================+
";

/// ANSI Game Over Scene
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
