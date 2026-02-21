use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct DevmojiEntry {
    pub code: String,
    pub emoji: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub devmoji: Vec<ConfigDevmojiEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDevmojiEntry {
    pub code: String,
    pub emoji: Option<String>,
    pub gitmoji: Option<String>,
    pub description: Option<String>,
}

pub static DEFAULT_TYPES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore", "build", "ci",
    ]
});

pub static DEFAULT_DEVMOJIS: Lazy<Vec<DevmojiEntry>> = Lazy::new(|| {
    vec![
        DevmojiEntry { code: "feat".into(), emoji: "sparkles".into(), description: "a new feature".into() },
        DevmojiEntry { code: "fix".into(), emoji: "bug".into(), description: "a bug fix".into() },
        DevmojiEntry { code: "docs".into(), emoji: "books".into(), description: "documentation only changes".into() },
        DevmojiEntry { code: "style".into(), emoji: "art".into(), description: "changes that do not affect the meaning of the code".into() },
        DevmojiEntry { code: "refactor".into(), emoji: "recycle".into(), description: "a code change that neither fixes a bug nor adds a feature".into() },
        DevmojiEntry { code: "perf".into(), emoji: "zap".into(), description: "a code change that improves performance".into() },
        DevmojiEntry { code: "test".into(), emoji: "rotating_light".into(), description: "adding missing or correcting existing tests".into() },
        DevmojiEntry { code: "chore".into(), emoji: "wrench".into(), description: "changes to the build process or auxiliary tools".into() },
        DevmojiEntry { code: "chore-release".into(), emoji: "rocket".into(), description: "code deployment or publishing to external repositories".into() },
        DevmojiEntry { code: "chore-deps".into(), emoji: "link".into(), description: "add or delete dependencies".into() },
        DevmojiEntry { code: "build".into(), emoji: "package".into(), description: "changes related to build processes".into() },
        DevmojiEntry { code: "ci".into(), emoji: "construction_worker".into(), description: "updates to the continuous integration system".into() },
        DevmojiEntry { code: "release".into(), emoji: "rocket".into(), description: "code deployment or publishing to external repositories".into() },
        DevmojiEntry { code: "security".into(), emoji: "lock".into(), description: "fixing security issues".into() },
        DevmojiEntry { code: "i18n".into(), emoji: "globe_with_meridians".into(), description: "internationalization and localization".into() },
        DevmojiEntry { code: "breaking".into(), emoji: "boom".into(), description: "introducing breaking changes".into() },
        DevmojiEntry { code: "config".into(), emoji: "gear".into(), description: "changing configuration files".into() },
        DevmojiEntry { code: "add".into(), emoji: "heavy_plus_sign".into(), description: "add something".into() },
        DevmojiEntry { code: "remove".into(), emoji: "heavy_minus_sign".into(), description: "remove something".into() },
    ]
});

pub struct Config {
    pub types: Vec<String>,
    pub devmojis: Vec<DevmojiEntry>,
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Self {
        let file_config = config_path
            .map(|p| PathBuf::from(p))
            .or_else(|| find_config_file())
            .and_then(|p| load_config_file(&p));

        let mut types: Vec<String> = DEFAULT_TYPES.iter().map(|s| s.to_string()).collect();
        let mut devmojis = DEFAULT_DEVMOJIS.clone();

        if let Some(cfg) = file_config {
            // Merge types
            for t in &cfg.types {
                if !types.contains(t) {
                    types.push(t.clone());
                }
            }

            // Merge devmoji entries
            for entry in &cfg.devmoji {
                let emoji = resolve_config_emoji(entry);
                let description = resolve_config_description(entry);

                if let Some(existing) = devmojis.iter_mut().find(|d| d.code == entry.code) {
                    if let Some(e) = &emoji {
                        existing.emoji = e.clone();
                    }
                    if let Some(d) = &description {
                        existing.description = d.clone();
                    }
                } else {
                    devmojis.push(DevmojiEntry {
                        code: entry.code.clone(),
                        emoji: emoji.unwrap_or_default(),
                        description: description.unwrap_or_default(),
                    });
                }
            }
        }

        Config { types, devmojis }
    }
}

fn resolve_config_emoji(entry: &ConfigDevmojiEntry) -> Option<String> {
    if let Some(emoji) = &entry.emoji {
        return Some(emoji.clone());
    }
    if let Some(gitmoji_code) = &entry.gitmoji {
        use crate::gitmoji::GITMOJI_MAP;
        use crate::gitmoji::GITMOJIS;
        if let Some(&idx) = GITMOJI_MAP.get(gitmoji_code.as_str()) {
            return Some(GITMOJIS[idx].code.to_string());
        }
    }
    None
}

fn resolve_config_description(entry: &ConfigDevmojiEntry) -> Option<String> {
    if let Some(desc) = &entry.description {
        return Some(desc.clone());
    }
    if let Some(gitmoji_code) = &entry.gitmoji {
        use crate::gitmoji::GITMOJI_MAP;
        use crate::gitmoji::GITMOJIS;
        if let Some(&idx) = GITMOJI_MAP.get(gitmoji_code.as_str()) {
            return Some(GITMOJIS[idx].description.to_string());
        }
    }
    None
}

fn find_config_file() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;

    // Check current directory
    let candidate = cwd.join("devmoji.config.json");
    if candidate.exists() {
        return Some(candidate);
    }

    // Walk up looking for package.json or .git
    let mut dir = cwd.as_path();
    loop {
        let candidate = dir.join("devmoji.config.json");
        if candidate.exists() {
            return Some(candidate);
        }

        // Check if this dir has package.json or .git
        if dir.join("package.json").exists() || dir.join(".git").exists() {
            let candidate = dir.join("devmoji.config.json");
            if candidate.exists() {
                return Some(candidate);
            }
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    // Check home directory
    if let Some(home) = dirs_home() {
        let candidate = home.join("devmoji.config.json");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
}

fn load_config_file(path: &Path) -> Option<ConfigFile> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}
