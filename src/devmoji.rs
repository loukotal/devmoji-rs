use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::{Config, DevmojiEntry};
use crate::github_emoji::GITHUB_EMOJIS;
use crate::gitmoji::GITMOJI_MAP;

static SHORTCODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r":([a-zA-Z0-9_\-+]+):").unwrap());
static SHORTCODE_SPACE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s?:([a-zA-Z0-9_\-+]+):").unwrap());

pub struct Devmoji {
    /// Maps devmoji code -> DevmojiEntry (code, emoji shortcode, description)
    pack: Vec<DevmojiEntry>,
    /// Maps shortcode (without colons) -> unicode emoji
    pack_map: HashMap<String, String>,
    /// Reverse map: unicode emoji -> shortcode
    emoji_to_code: HashMap<String, String>,
}

impl Devmoji {
    pub fn new(config: &Config) -> Self {
        let pack = config.devmojis.clone();
        let mut pack_map = HashMap::new();
        for entry in &pack {
            pack_map.insert(entry.code.clone(), entry.emoji.clone());
        }

        // Build reverse map from unicode emoji to shortcode (github emojis)
        let mut emoji_to_code: HashMap<String, String> = HashMap::new();
        for (&code, &emoji) in GITHUB_EMOJIS.iter() {
            emoji_to_code
                .entry(emoji.to_string())
                .or_insert_with(|| code.to_string());
            // Also store without variation selector
            let stripped = emoji.replace('\u{fe0f}', "");
            if stripped != emoji {
                emoji_to_code
                    .entry(stripped)
                    .or_insert_with(|| code.to_string());
            }
        }

        Devmoji {
            pack,
            pack_map,
            emoji_to_code,
        }
    }

    pub fn pack(&self) -> &[DevmojiEntry] {
        &self.pack
    }

    /// Resolve a code to its unicode emoji.
    /// First checks pack (devmoji aliases), then github emojis.
    pub fn get(&self, code: &str) -> String {
        // Check if it's a devmoji pack code -> resolve to its emoji shortcode, then recurse
        if let Some(emoji_code) = self.pack_map.get(code) {
            if emoji_code != code {
                return self.get(emoji_code);
            }
        }

        // Check github emoji registry
        if let Some(&emoji) = GITHUB_EMOJIS.get(code) {
            return emoji.to_string();
        }

        // Not found - return wrapped
        format!(":{}:", code)
    }

    /// Convert unicode emoji to shortcodes
    pub fn demojify(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());

        for ch in text.chars() {
            let s = ch.to_string();
            // Try with variation selector
            let with_vs = format!("{}\u{fe0f}", ch);

            if let Some(code) = self
                .emoji_to_code
                .get(&s)
                .or_else(|| self.emoji_to_code.get(&with_vs))
            {
                // Skip variation selectors themselves
                if ch == '\u{fe0f}' {
                    continue;
                }
                result.push_str(&format!(":{}:", code));
            } else if ch == '\u{fe0f}' {
                // Skip standalone variation selectors
                continue;
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Convert shortcodes to unicode emoji
    pub fn emojify(&self, text: &str) -> String {
        // First normalize to shortcodes
        let text = self.demojify(text);
        // Then resolve shortcodes to unicode
        SHORTCODE_RE
            .replace_all(&text, |caps: &regex::Captures| {
                let code = &caps[1];
                self.get(code)
            })
            .to_string()
    }

    /// Strip all emoji from text
    pub fn strip(&self, text: &str) -> String {
        let text = self.demojify(text);
        SHORTCODE_SPACE_RE.replace_all(&text, "").to_string()
    }

    /// Convert to devmoji shortcodes (custom aliases)
    pub fn devmojify(&self, text: &str) -> String {
        let text = self.demojify(text);
        SHORTCODE_RE
            .replace_all(&text, |caps: &regex::Captures| {
                let code = &caps[1];
                // Look up the github emoji for this code
                if let Some(&emoji) = GITHUB_EMOJIS.get(code) {
                    // Check if any devmoji pack entry maps to this emoji
                    for entry in &self.pack {
                        let resolved = self.resolve_pack_emoji(&entry.emoji);
                        if resolved == emoji {
                            return format!(":{}:", entry.code);
                        }
                    }
                }
                // Also check gitmoji
                if GITMOJI_MAP.contains_key(code) {
                    for entry in &self.pack {
                        if entry.emoji == code {
                            return format!(":{}:", entry.code);
                        }
                    }
                }
                format!(":{}:", code)
            })
            .to_string()
    }

    fn resolve_pack_emoji(&self, emoji_code: &str) -> String {
        if let Some(&emoji) = GITHUB_EMOJIS.get(emoji_code) {
            return emoji.to_string();
        }
        emoji_code.to_string()
    }
}
