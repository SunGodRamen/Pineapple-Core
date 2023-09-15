# Pineapple-Core

## Roadmap v0.1.0:
Directory structure:

modules/core
├── Cargo.toml
├── README.md
└── src
    ├── file_interface
    │   ├── mod.rs
    │   ├── file_browser.rs
    │   └── file_launcher.rs
    ├── tcp_interface
    │   ├── mod.rs
    │   ├── server.rs
    │   └── client.rs
    ├── context_system
    │   ├── mod.rs
    │   ├── db.rs
    │   └── context.rs
    ├── configuration_system
    │   ├── mod.rs
    │   ├── states.rs
    │   └── events.rs
    ├── script_library
    │   ├── mod.rs
    │   ├── file_handlers.rs
    │   ├── window_handlers.rs
    │   └── hotkey_handlers.rs
    ├── cli
    │   ├── mod.rs
    │   └── commands.rs
    └── main.rs


Explanation:

main.rs:

The main entry point of the Rust application.
Initializes the core systems, possibly starts the main event loop, and orchestrates high-level app behavior.

file_interface: This module handles the interaction with the file system, including browsing and launching files.

    file_browser.rs:
    This is part of the file_interface module.
    Responsible for browsing the filesystem and retrieving relevant file information.
    Lists directories and files, possibly with filtering capabilities based on file types.
    Retrieves file metadata like name, path, type, and thumbnail.

    file_launcher.rs:
    Also part of the file_interface module.
    Manages the opening of files with default or specified applications.
    Might handle batch opening of files and special use cases like opening files with non-default apps.

context_system: Handles the tracking of open file windows and their contexts. The db.rs might interact with your chosen database/storage mechanism (for example, SQLite), while context.rs would contain structs and functions related to different contexts.

    db.rs:
    Part of the context_system module.
    Interfaces with the chosen database or storage system, likely handling CRUD operations.
    Would be responsible for maintaining a lightweight database of active windows and their contexts.

    context.rs:
    Also part of the context_system module.
    Contains structs and functions related to different contexts, tying together window data with associated metadata.
    Might also handle interactions with the configuration_system.


configuration_system: Manages the system states and events which trigger scripts. states.rs could store various state configurations, and events.rs could store event-triggered actions or handlers.

    states.rs:
    Part of the configuration_system module.
    Defines various system states.
    Can store configurations that the system can be in, or states that trigger certain scripts.
    
    events.rs:
    Also part of the configuration_system module.
    Manages events that might trigger state changes or script executions.
    Defines event-driven behavior in the system.


script_library: A collection of scripts and handlers. These scripts could interact with the context_system and configuration_system. This module can be expanded further based on the diversity of scripts you'll be handling. For instance, file_handlers.rs might contain scripts that handle file operations, window_handlers.rs for window positioning and sizing, and hotkey_handlers.rs for scripts related to hotkey configurations.

    file_handlers.rs:
    Within the script_library module.
    Contains scripts/functions specific to file operations like copy, move, delete, etc.
    
    window_handlers.rs:
    Another part of the script_library module.
    Contains scripts/functions related to window operations, like positioning, sizing, or state (minimize/maximize/close).

    hotkey_handlers.rs:
    Also in the script_library module.
    Manages scripts/functions related to hotkey configurations and actions triggered by hotkeys.

cli: This would be the command-line interface module if you choose to have command line functionality outside of the GUI.

    commands.rs:
    Belongs to the cli module.
    Defines commands for the CLI and their behavior, like parsing and execution.
