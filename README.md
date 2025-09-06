# blink

**Fast, practical fuzzy search for files and projects.**

A command-line tool built with Rust that provides instant file discovery using `fd` and `fzf` with sensible defaults and minimal configuration.

## Features

* ✅ Interactive file search with fuzzy matching
* ✅ Project-aware navigation 
* ✅ Configurable search patterns and exclusions
* ✅ Editor integration
* ✅ Lightweight and fast

## Installation

### Prerequisites

Make sure you have the following tools installed:

* `fd` - Fast file finder (replaces `find`)
* `fzf` - Fuzzy finder for interactive selection
* An editor (configurable, defaults to `$EDITOR` environment variable)

On Ubuntu/Debian:
```bash
sudo apt install fd-find fzf
```

On macOS:
```bash
brew install fd fzf
```

On Arch Linux:
```bash
sudo pacman -S fd fzf
```

### Build from source

```bash
git clone https://github.com/your-username/blink.git
cd blink
cargo build --release
sudo cp target/release/blink /usr/local/bin/
```

## Usage

### Commands

All commands are argument-free for maximum simplicity. Configuration is handled via the config file.

```bash
# Interactive file search (all files except excluded same as gf)
blink

# Interactive file search (all files except excluded)
blink gf

# Global content search (not yet implemented)
blink gs

# Find project directories (not yet implemented) 
blink pf

# Search within project scope (not yet implemented)
blink ps

# Find media files (not yet implemented)
blink ms

# Find documents (not yet implemented)
blink df
```

### Help

```bash
# General help
blink --help

# Command-specific help
blink gf --help
```

## Configuration
Create a configuration file at `~/.config/blink/config.toml`:

```toml
# Favorite directories for quick navigation
fav_dirs = [
    "~/Projects",
    "~/work",
    "~/.config",
    "~/brain",
]

# Patterns to exclude from search results
exclude = [
    "*.git*",
    "node_modules",
    "target",
    "build",
    "dist",
    "*.tmp",
    "*.log",
    "*.cache"
]

# Default editor (fallback to $EDITOR if not set)
editor = "nvim"

# File patterns that indicate project root directories
project_markers = [
".git", ".hg", ".svn", "Makefile", "CMakeLists.txt", "pom.xml", "build.gradle", "build.gradle.kts", 
 "Cargo.toml", "setup.py", "pyproject.toml", "requirements.txt", "Gemfile", "Mix.exs", "rebar.config", 
 "project.clj", "deps.edn", "shard.yml", "pubspec.yaml", "elm.json", "Project.toml", "dune-project", 
 "xmake.lua", "SConstruct", "meson.build", "WORKSPACE", "default.nix", "flake.nix", "TAGS", "GTAGS"
]

# Maximum search depth
max_depth = 10

# Maximum file size for previews in bytes
preview_max_size = 1048576
```

## Commands

### Currently Implemented

- `gf` - Global file search using fd and fzf with interactive selection

### Planned Features

- `gs` - Global content search using ripgrep  
- `pf` - Project finder (search for project roots)
- `ps` - Project-scoped search
- `ms` - Media file search with preview
- `df` - Document search and viewer integration

### Design Philosophy

All commands are argument-free to minimize friction. Configuration drives behavior instead of command-line flags. This approach prioritizes speed and simplicity in daily usage over flexibility in individual commands.

## Development

### Project Structure

```
blink/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command-line interface definitions
│   ├── config.rs        # Configuration loading and management
│   └── commands/
│       ├── mod.rs       # Commands module
│       └── global_find.rs # Global file search implementation
├── examples/
│   └── config.toml      # Example configuration file
└── README.md
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run -- gf
```

### Testing

```bash
# Run tests
cargo test

# Run with specific pattern
cargo test global_find
```

## Dependencies

- `clap` - Command-line argument parsing
- `serde` - Serialization/deserialization for config
- `toml` - TOML configuration file parsing
- `dirs` - Cross-platform directories discovery

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- Inspired by the need for better file navigation tools
- Built on top of excellent Rust ecosystem tools
- Thanks to the `fd` and `fzf` projects for providing the underlying functionality
