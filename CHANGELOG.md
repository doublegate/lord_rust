# Changelog

All notable changes to the Legend of the Red Dragon (L.O.R.D.) Rust recreation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0]

### Added - Overall

## Legend of the Red Dragon (Rust Edition) - Version 0.2.0

## Core Features

### 1. Player Management System

- **Character Creation and Authentication**
  - Technical Implementation:
    - Secure password storage using Argon2 hashing algorithm
    - Player data stored in PostgreSQL database
    - Username uniqueness enforced at database level
    - Optional password authentication

- **Player Statistics and Progression**
  - Technical Implementation:
    - Player struct with comprehensive attributes (HP, attack, defense, etc.)
    - Experience-based leveling system with automatic stat increases
    - Daily reset mechanism for forest fights and player revival
    - Persistent player state across game sessions

### 2. Combat Systems

- **Forest Exploration and Monster Battles**
  - Technical Implementation:
    - Random monster generation scaled to player level
    - Turn-based combat system with attack/defense mechanics
    - Monster templates with varying difficulty levels
    - Experience and gold rewards based on monster strength
    - Limited daily forest fights (10 per day)
    - Death handling with news event logging

- **Player vs Player (PvP) Combat**
  - Technical Implementation:
    - Challenge system to select opponents from active players
    - Turn-based duel mechanics similar to monster combat
    - Gold looting from defeated opponents (50% of their gold)
    - Experience rewards based on opponent level
    - Death handling with news event logging
    - Protection system preventing multiple defeats of the same player

### 3. Social and Role-Playing Elements

- **Romance System with NPC**
  - Technical Implementation:
    - Progressive romance with Violet the barmaid
    - Five-stage romance progression with unique dialogue
    - Marriage system with public announcement
    - Spouse status tracking in player data

- **Tavern Interactions**
  - Technical Implementation:
    - Multiple tavern activities (flirting, drinking, gossip)
    - Healing mechanism through purchasing drinks
    - News/gossip system showing recent game events
    - ANSI art for visual immersion

### 4. Game World and Interface

- **Town Hub with Multiple Activities**
  - Technical Implementation:
    - Central menu system with 7 main options
    - Player status display with current stats
    - Navigation to all game areas (forest, tavern, duels)
    - Character information display
    - Leaderboard system

- **ANSI Text-Based User Interface**
  - Technical Implementation:
    - Colored text using the `colored` crate
    - ASCII art for different game areas (title, forest, tavern, town)
    - Terminal screen clearing for better readability
    - Input prompting system with standardized format

### 5. Data Persistence and Management

- **PostgreSQL Database Integration**
  - Technical Implementation:
    - SQLx for type-safe asynchronous database operations
    - Connection pooling with configurable pool size (10 connections)
    - Environment variable configuration with dotenvy
    - Daily reset mechanism for game state
    - Transaction support for atomic operations

- **Game Event Logging**
  - Technical Implementation:
    - News system recording significant game events
    - Timestamped entries for player achievements
    - Combat outcomes, level-ups, and marriages logged
    - Viewable through town menu or tavern gossip

## Technical Architecture

### 1. Database Layer (`src/db/mod.rs`)

- **Schema Design**
  - Players table with comprehensive character attributes
  - News table for event logging
  - Game state table for global settings

- **Database Operations**
  - Player creation, retrieval, and updating
  - Password hashing and verification
  - Daily reset functionality
  - Event logging and retrieval
  - Player listing and leaderboard queries

### 2. Game Logic (`src/game/`)

- **Core Game Structures (`mod.rs`)**
  - Player struct definition
  - Game constants (e.g., MAX_DAILY_FOREST_FIGHTS)
  - Level-up mechanics and experience calculations

- **Forest Module (`forest.rs`)**
  - Monster generation and scaling
  - Combat simulation
  - Reward distribution
  - Death handling

- **PvP Module (`pvp.rs`)**
  - Opponent selection
  - Duel simulation
  - Victory/defeat handling
  - Gold and experience rewards

- **Romance Module (`romance.rs`)**
  - Flirting progression system
  - Marriage trigger conditions
  - Tavern drink healing mechanics
  - Gossip/news display

- **Town Module (`town.rs`)**
  - Main menu implementation
  - Navigation to other game areas
  - Character information display
  - Leaderboard functionality

### 3. User Interface (`src/ui/`)

- **Input/Output Utilities (`mod.rs`)**
  - User input prompting
  - Screen clearing
  - Title display

- **ANSI Art (`ansi_art.rs`)**
  - ASCII art for different game areas
  - Colored text using ANSI escape codes
  - Visual representation of game locations

### 4. Main Program (`src/main.rs`)

- **Application Entry Point**
  - Database initialization
  - Daily reset check
  - User authentication flow
  - Main game loop orchestration

## Dependencies and Technologies

- **Rust Crates**
  - `sqlx`: PostgreSQL database connectivity with async support
  - `tokio`: Asynchronous runtime for database operations
  - `chrono`: Date and time handling for game events and resets
  - `argon2`: Secure password hashing
  - `rand`: Random number generation for combat and encounters
  - `colored`: Terminal text coloring
  - `dotenvy`: Environment variable management

- **Database Technology**
  - PostgreSQL with connection pooling
  - Type-safe SQL queries
  - Transaction support for data integrity

This version (0.2.0) represents a significant upgrade from the previous version, with a focus on robust database integration using PostgreSQL, asynchronous operations with Tokio, and improved error handling throughout the application.

### Changed - Overall

### Deprecated

### Removed

### Fixed

### Security

## [0.2.0] - 2025-04-15

### Added - ver. 0.2

- Robust database layer using SQLx with PostgreSQL
- Asynchronous database operations with Tokio runtime
- Migration system for database schema versioning
- Entity-relationship model for game objects
- Connection pooling for efficient database access
- Environment variable configuration with dotenvy
- Type-safe SQL queries with compile-time verification
- Transaction support for atomic operations
- Error handling and logging for database operations
- Unit and integration tests for database layer

### Changed - ver. 0.2

- Refactored data access layer to use repository pattern
- Improved error handling with custom error types
- Enhanced database connection management

### Technical Details - ver. 0.2

- Implemented SQLx with PostgreSQL for data persistence
- Set up connection pooling with configurable pool size
- Created migration scripts for schema evolution
- Configured Tokio runtime for async database operations
- Implemented prepared statements for SQL injection prevention
- Added database transaction support for data integrity
- Created database models with proper relationships
- Set up environment-based configuration for development/production
- Implemented query macros for compile-time SQL verification
- Added database health checks and connection validation

## [0.1.0] - 2025-03-08

### Added - ver. 0.1

- Initial release
- Basic game mechanics
- Character creation system
- Combat system
- Forest encounters
- Village functionality

[0.2.0]: https://github.com/doublegate/lord_rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/doublegate/lord_rust/releases/tag/v0.1.0
