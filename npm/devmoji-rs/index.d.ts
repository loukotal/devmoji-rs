export interface DevmojiEntry {
  /** The shortcode for this devmoji (e.g. "feat", "fix", "chore-deps") */
  code: string;
  /** GitHub emoji shortcode to use (e.g. "sparkles", "bug"). Takes precedence over `gitmoji`. */
  emoji?: string;
  /** Gitmoji code to resolve the emoji from (e.g. "construction"). Used as fallback if `emoji` is not set. */
  gitmoji?: string;
  /** Description of what this devmoji represents */
  description?: string;
}

export interface DevmojiConfig {
  /** Additional conventional commit types beyond the defaults (feat, fix, docs, style, refactor, perf, test, chore, build, ci) */
  types?: string[];
  /** Custom devmoji entries. Entries with the same `code` as a default will override it. */
  devmoji?: DevmojiEntry[];
}

/**
 * Define a devmoji configuration with full type support.
 *
 * @example
 * ```ts
 * // devmoji.config.ts
 * import { defineDevmojiConfig } from '@loukotal/devmoji-rs';
 *
 * export default defineDevmojiConfig({
 *   devmoji: [
 *     { code: "fix", emoji: "saluting_face" },
 *     { code: "wip", gitmoji: "construction", description: "work in progress" },
 *   ],
 * });
 * ```
 */
export function defineDevmojiConfig(config: DevmojiConfig): DevmojiConfig;
