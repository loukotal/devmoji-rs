# devmoji-rs

Rust port of [devmoji](https://github.com/folke/devmoji) by [@folke](https://github.com/folke). ~10x faster startup.

## Install

```sh
cargo install --git https://github.com/loukotal/devmoji-rs
```

## Usage

```sh
# Add emoji to conventional commits
echo "feat: add login" | devmoji
# feat: ✨ add login

# As a git hook (prepare-commit-msg)
devmoji -e

# List all available emoji codes
devmoji --list

# Custom config
echo '{"devmoji": [{"code": "fix", "emoji": "saluting_face"}]}' > devmoji.config.json
```

## Credits

All credit for the original concept, design, and emoji mappings goes to [devmoji](https://github.com/folke/devmoji) by [Folke Lemaitre](https://github.com/folke).
