# Changelog

All notable changes to the Legend of the Red Dragon (L.O.R.D.) Rust recreation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - Legend of the Red Dragon (Rust Edition) - Version 0.3.0

## Changes and Features Since Last Version (v0.2.0 to v0.3.0)

## Overall Program Changes

### 1. Database Migration from SQLite to PostgreSQL

- **Technical Implementation:**
  - Replaced `rusqlite` with `sqlx` for PostgreSQL support
  - Implemented connection pooling for efficient database access
  - Added type-safe SQL queries with compile-time verification
  - Created migration system for database schema versioning
  - Implemented transaction support for atomic operations

### 2. Authentication System Enhancement

- **Technical Implementation:**
  - Integrated Argon2 password hashing algorithm for improved security
  - Implemented password verification system
  - Added username uniqueness enforcement at database level
  - Created optional password authentication flow

### 3. Asynchronous Runtime Integration

- **Technical Implementation:**
  - Added Tokio runtime for asynchronous database operations
  - Implemented non-blocking I/O for improved performance
  - Created async/await patterns throughout database layer
  - Configured runtime with appropriate thread pool settings

### 4. Environment Configuration System

- **Technical Implementation:**
  - Added `dotenvy` for environment variable management
  - Created configurable database connection parameters
  - Implemented environment-based configuration for development/production
  - Added database health checks and connection validation

## Game Features and Mechanics

### 1. Enhanced Player Management System

- **Technical Implementation:**
  - Redesigned Player struct with comprehensive attributes
  - Implemented persistent player state across game sessions
  - Created daily reset mechanism for forest fights and player revival
  - Added experience-based leveling system with automatic stat increases

### 2. Improved Combat Systems

- **Technical Implementation:**
  - Enhanced random monster generation scaled to player level
  - Refined turn-based combat system with attack/defense mechanics
  - Created monster templates with varying difficulty levels
  - Implemented experience and gold rewards based on monster strength
  - Added death handling with news event logging

### 3. Expanded PvP Combat

- **Technical Implementation:**
  - Developed challenge system to select opponents from active players
  - Created turn-based duel mechanics similar to monster combat
  - Implemented gold looting from defeated opponents (50% of their gold)
  - Added experience rewards based on opponent level
  - Created protection system preventing multiple defeats of the same player

### 4. Enhanced Social and Role-Playing Elements

- **Technical Implementation:**
  - Developed progressive romance with Violet the barmaid
  - Created five-stage romance progression with unique dialogue
  - Implemented marriage system with public announcement
  - Added spouse status tracking in player data
  - Enhanced tavern interactions (flirting, drinking, gossip)

### 5. Improved Game World and Interface

- **Technical Implementation:**
  - Refined central menu system with 7 main options
  - Enhanced player status display with current stats
  - Improved navigation to all game areas (forest, tavern, duels)
  - Updated character information display
  - Enhanced leaderboard system

### 6. Advanced Data Persistence and Management

- **Technical Implementation:**
  - Integrated PostgreSQL database with SQLx
  - Implemented connection pooling with configurable pool size
  - Created environment variable configuration with dotenvy
  - Enhanced daily reset mechanism for game state
  - Added transaction support for atomic operations

## Technical Architecture Improvements

### 1. Database Layer Enhancements

- **Technical Implementation:**
  - Redesigned schema with comprehensive character attributes
  - Created news table for event logging
  - Added game state table for global settings
  - Implemented repository pattern for data access
  - Enhanced error handling with custom error types

### 2. Game Logic Refinements

- **Technical Implementation:**
  - Improved core game structures and constants
  - Enhanced level-up mechanics and experience calculations
  - Refined monster generation and scaling
  - Improved combat simulation and reward distribution
  - Enhanced PvP opponent selection and duel simulation

### 3. User Interface Improvements

- **Technical Implementation:**
  - Enhanced user input prompting
  - Improved screen clearing and display
  - Refined title display and ASCII art
  - Added colored text using ANSI escape codes
  - Created visual representations of game locations

## Dependencies and Technologies Updates

- **Updated Rust Crates:**
  - Upgraded to `sqlx` v0.8.3 with PostgreSQL support
  - Updated to `tokio` v1 with full feature set
  - Added `argon2` v0.5 for password hashing
  - Updated `rand` to v0.9.0
  - Added `dotenvy` v0.15.7 for environment management
  - Updated `colored` to v3.0.0
  - Added `chrono` with serde features for date/time handling

## [0.2.0] - Legend of the Red Dragon (Rust Edition) - Version 0.2.0

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
