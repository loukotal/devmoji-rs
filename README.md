# devmoji-rs

Rust port of [devmoji](https://github.com/folke/devmoji) by [@folke](https://github.com/folke). ~10x faster startup.

## Install

```sh
# npm
npm install -D @loukotal/devmoji-rs

# pnpm
pnpm add -D @loukotal/devmoji-rs

# yarn
yarn add -D @loukotal/devmoji-rs

# cargo
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

# Custom config (JSON)
echo '{"devmoji": [{"code": "fix", "emoji": "saluting_face"}]}' > devmoji.config.json
```

### Configuration

devmoji looks for a config file by walking up the directory tree. Supported formats:

- `devmoji.config.json` — plain JSON
- `devmoji.config.js` / `devmoji.config.mjs` — ES module (requires Node.js)
- `devmoji.config.ts` / `devmoji.config.mts` — TypeScript (requires [tsx](https://github.com/privatenumber/tsx) or Node.js >= 22.6)

#### TypeScript config with `defineDevmojiConfig`

```ts
// devmoji.config.ts
import { defineDevmojiConfig } from '@loukotal/devmoji-rs';

export default defineDevmojiConfig({
  types: ['wip'],
  devmoji: [
    { code: 'fix', emoji: 'saluting_face' },
    { code: 'wip', gitmoji: 'construction', description: 'work in progress' },
  ],
});
```

#### JavaScript config

```js
// devmoji.config.js
export default {
  devmoji: [
    { code: 'fix', emoji: 'saluting_face' },
  ],
};
```

## Credits

All credit for the original concept, design, and emoji mappings goes to [devmoji](https://github.com/folke/devmoji) by [Folke Lemaitre](https://github.com/folke).
