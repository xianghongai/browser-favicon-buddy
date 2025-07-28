# Browser Favicon Buddy - AI Coding Agent Instructions

This document provides essential information to help AI coding agents understand and contribute to the Browser Favicon Buddy project.

## Project Overview

Browser Favicon Buddy is a cross-platform GUI application written in Rust that processes browser bookmark files (HTML) to add favicon icons. It supports multiple languages (internationalization) and configurable favicon service providers.

## Architecture

The project follows a modular architecture:

- **Main Components**:
  - `src/main.rs`: Entry point, initializes i18n and launches UI
  - `src/lib.rs`: Defines the module structure
  - `src/favicon/`: Core functionality for processing bookmark files and favicon handling
  - `src/ui/`: GUI components using egui/eframe
  - `src/config/`: Configuration management
  - `src/i18n.rs`: Internationalization system

## Key Workflows

### Build & Run

```bash
# Development build
cargo run

# Release build
cargo build --release

# Cross-compilation (requires Cross)
cross build --target x86_64-pc-windows-gnu --release
```

### Favicon Processing Flow

1. User selects a browser bookmark HTML file via GUI
2. System extracts all bookmark URLs from the HTML
3. For each domain, system checks favicon cache or fetches from configured service
4. HTML is modified to include base64-encoded favicon data
5. Output file is saved with "-with-favicons--<timestamp>" suffix

## Project-Specific Patterns

### Internationalization

- Uses Fluent syntax (FTL) converted from YAML
- Default language: `zh-CN`, also supports `en`
- Usage pattern: `crate::i18n::get_message("key", optional_args_hashmap)`

```rust
// Example with arguments
let mut args = std::collections::HashMap::new();
args.insert("path".to_string(), path.to_string());
let message = crate::i18n::get_message("selected_file", Some(args));
```

### Configuration System

- Config stored in user's home directory under `.config/favicon-buddy/config.json`
- Services for favicon retrieval can be added/edited/removed
- Configuration serialized with serde

### UI Components

- Built with egui/eframe
- Components defined in `src/ui/components/`
- UI state management via `AppState` struct

### Async Processing

- Uses Tokio for async operations
- Maintains progress via shared atomic variables and mutex

## External Dependencies

- **GUI**: eframe/egui
- **HTTP**: reqwest with tokio
- **HTML**: scraper/regex
- **Serialization**: serde, serde_json, serde_yaml
- **Internationalization**: fluent, fluent-bundle

## Common Tasks

### Adding a New Favicon Service

1. Edit `src/config/favicon_service.rs`
2. Add new entry to the services vector with name and URL template

### Adding New UI Component

1. Create new file in `src/ui/components/`
2. Define rendering function
3. Integrate in `app_state.rs`

### Adding New Translation

1. Add key/value in YAML files in `locales/`
2. Access via `i18n::get_message()` API

### Handling Errors

- Error types defined in `src/errors.rs`
- Use `AppResult<T>` type alias for functions that can fail
