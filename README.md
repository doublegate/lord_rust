# Legend of the Red Dragon (Rust Edition)

**Legend of the Red Dragon (LORD)** is a classic text-based BBS door game from 1989. This is a 100% faithful fan-made recreation in Rust, aiming to replicate the original game's mechanics and charm. It's a single-player/multi-user turn-based RPG that runs in a terminal, complete with ANSI colors and ASCII art for a nostalgic experience.

## Features

- **Forest Fights:** Battle monsters in the forest to gain experience and gold. You have a limited number of fights each day, which reset daily.
- **Leveling System:** Gain levels as you accumulate experience. Level-ups increase your health, attack, and defense automatically.
- **Player vs Player (PvP) Duels:** Challenge other players to duels. Victors can earn a portion of the defeated player's gold. (Defeated players cannot be attacked again until the next day.)
- **Daily Resets:** Every real-life day, the game refreshes: players are healed to full, daily fights are restored, and fallen players can play again. This simulates the traditional BBS daily turn cycle.
- **Romance and Tavern:** Interact with Violet, the barmaid, in the tavern. Flirt to build romance; if you charm her enough, you might get married! You can also buy drinks to heal and listen to gossip (the daily news).
- **Town Menu:** A hub of actions including exploring the forest, visiting the tavern, dueling other players, viewing character stats, reading the daily news log, and checking the hero leaderboard.
- **Persistent Game State:** All player data and game events are stored in a local SQLite database (`lord.sqlite`), so progress and history are saved between sessions.
- **ANSI Text Interface:** The game uses ANSI escape codes to provide color and simple text-based art, recreating the feel of the original LORD interface. (You can disable ANSI in your terminal if needed.)

## Setup and Running

1. **Install Rust:** Ensure you have Rust and Cargo installed (edition 2021 or later).
2. **Clone this repository:** Download the code or clone the GitHub repo.
3. **Build the project:** Run `cargo build --release` to compile the game (release build is recommended for performance).
4. **Run the game:** Use `cargo run` (or `./target/release/legend_of_the_red_dragon` after building). A SQLite database file will be created in the current directory to store game data.
5. **Gameplay:** Follow the on-screen prompts. On first run, you'll be asked to create a character. Use the numbered menu to navigate:
   - `1` = Enter the Forest (fight monsters)
   - `2` = Visit the Tavern (romance, heal, gossip)
   - `3` = Duel another player (PvP combat)
   - `4` = View your character (stats and info)
   - `5` = Read Daily News (recent game events)
   - `6` = Leaderboard (top players by level)
   - `7` = Save and Quit

   Use these options to adventure, and remember you get new opportunities every new day!

### Using a Remote Database

The game now supports connecting to a remote SQLite database hosted on a server:

1. **Set up a SQLite server:** You'll need a server that exposes SQLite over HTTP/HTTPS using a compatible protocol.
2. **Configure the connection:** Set the `DATABASE_URL` environment variable to point to your remote database:

   ```sh
   # For Windows
   set DATABASE_URL=sqlite+http://username:password@server.example.com:8080/path/to/database
   
   # For Linux/macOS
   export DATABASE_URL=sqlite+http://username:password@server.example.com:8080/path/to/database
   ```

3. **Run the game:** The game will automatically connect to the remote database instead of using a local file.

This feature allows multiple players to share the same game world even when playing from different locations.

## Project Structure

- **Cargo.toml:** Rust package configuration, listing dependencies like `rusqlite` for database, `chrono` for date/time, `rand` for random number generation, and `colored` for colored terminal text.
- **src/main.rs:** Program entry. Handles initialization (database setup, daily reset), user login/registration, and the main game loop.
- **src/db/**: Database layer.
  - `mod.rs`: Defines the SQLite schema (players table, news log table, etc.) and functions to create or update players and log events. Uses `rusqlite`.
- **src/game/**: Game logic.
  - `mod.rs`: Contains core game structures and constants (e.g. `Player` struct and daily limits). Also utility functions like leveling up.
  - `forest.rs`: Forest exploration and monster encounter logic (random monsters, fight mechanics).
  - `pvp.rs`: Player vs player combat logic (target selection and duel simulation).
  - `romance.rs`: Tavern interactions (flirting with Violet, drinking for health, reading gossip).
  - `town.rs`: Implements the main town menu, routing the player's choices to the appropriate game actions.
- **src/ui/**: User interface and presentation.
  - `mod.rs`: Terminal I/O helpers (reading input, printing output, clearing screen) and a function to display the title banner.
  - `ansi_art.rs`: Contains ANSI escape code strings for the title screen and any other ASCII art or colored text banners.

Each module is documented and organized for clarity. The code is written to be as close to the original game's behavior as possible, but in a modern Rust context.

## Contributing

Contributions are welcome! If you have ideas for new features (additional areas, items, classes, etc.) or improvements (better ANSI art, refactoring), feel free to open an issue or pull request. The modular structure should make it easy to add new functionality:

- Add new commands in the town menu by updating `town.rs`.
- Implement new game mechanics in a separate module under `src/game` and integrate it accordingly.
- Update the database schema in `db/mod.rs` if persistent storage for new features is needed (e.g., a new table for messages or an inventory system).

Please test any changes thoroughly. This project is meant for fun and nostalgia – let's keep it that way by ensuring stability and simplicity.

**Note:** This is a fan project. Legend of the Red Dragon is a trademark of its original creator. This reimplementation is for educational and nostalgic purposes under fair use. Enjoy slaying monsters and romancing barmaids, just like the good old days!
