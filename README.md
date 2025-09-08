# Blink

Fast, practical fuzzy search for files and strings.

## Features

- Interactive fuzzy file search
- Project-aware navigation
- Configurable search options
- Editor integration

## Installation

### Prerequisites
- `fd`, `fzf`, `ripgrep`

### Package Managers
NOT YET DONE! INSTALL FROM SOURCE!

**Arch Linux:**
```bash
sudo pacman -S blink
```

**Cargo:**
```bash
cargo install blink
```

**macOS:**
```bash
brew install blink
```

### From Source
```bash
git clone https://github.com/anrichtait/blink.git
cd blink
cargo build --release
sudo cp target/release/blink /usr/local/bin/
```

## Usage

```bash
# Global file search
blink gf

# Help
blink --help
blink gf --help
```

## Configuration

See the example config for options: `cp examples/config.toml ~/.config/blink/config.toml`:

## Commands

- `gf` - Global file search (implemented)
- `gp` - Global project search (planned)
- `gs` - Global string search (planned)
- `pf` - Project file search (planned)
- `ps` - Project string search (planned)

## Development

```bash
cargo build
cargo test
RUST_LOG=debug cargo run -- gf
```

## License

:TODO:

---
