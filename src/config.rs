use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::{Path, PathBuf};
use std::process::Command;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DevmojiEntry {
    pub code: String,
    pub emoji: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ConfigFile {
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(default)]
    pub devmoji: Vec<ConfigDevmojiEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigDevmojiEntry {
    pub code: String,
    pub emoji: Option<String>,
    pub gitmoji: Option<String>,
    pub description: Option<String>,
}

/// All supported config file extensions, in priority order.
const CONFIG_EXTENSIONS: &[&str] = &["json", "ts", "mts", "js", "mjs"];

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

/// Check if any config file exists in a directory (across all supported extensions).
fn find_config_in_dir(dir: &Path) -> Option<PathBuf> {
    for ext in CONFIG_EXTENSIONS {
        let candidate = dir.join(format!("devmoji.config.{}", ext));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn find_config_file() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;

    // Check current directory
    if let Some(found) = find_config_in_dir(&cwd) {
        return Some(found);
    }

    // Walk up looking for config files
    let mut dir = cwd.as_path();
    loop {
        if let Some(found) = find_config_in_dir(dir) {
            return Some(found);
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    // Check home directory
    if let Some(home) = dirs_home() {
        if let Some(found) = find_config_in_dir(&home) {
            return Some(found);
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
    let ext = path.extension()?.to_str()?;
    match ext {
        "json" => load_json_config(path),
        "js" | "mjs" | "ts" | "mts" => load_js_config(path),
        _ => None,
    }
}

fn load_json_config(path: &Path) -> Option<ConfigFile> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

/// Cached config entry stored in `node_modules/.cache/devmoji/config.json`.
#[derive(Debug, Serialize, Deserialize)]
struct CachedConfig {
    /// Hash of the source config file contents.
    source_hash: u64,
    /// The resolved config.
    config: ConfigFile,
}

/// Compute a hash of the given bytes using the standard library hasher.
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

/// Find the `node_modules/.cache/devmoji/` directory relative to the config file.
/// Walks up from the config file's parent looking for `node_modules/`.
fn find_cache_dir(config_path: &Path) -> Option<PathBuf> {
    let start = config_path.parent()?;
    let mut dir = start;
    loop {
        let nm = dir.join("node_modules");
        if nm.is_dir() {
            return Some(nm.join(".cache").join("devmoji"));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

/// Try to read a cached config. Returns `Some` if the cache exists and the
/// source hash matches.
fn read_cached_config(cache_dir: &Path, source_hash: u64) -> Option<ConfigFile> {
    let cache_file = cache_dir.join("config.json");
    let contents = std::fs::read_to_string(&cache_file).ok()?;
    let cached: CachedConfig = serde_json::from_str(&contents).ok()?;
    if cached.source_hash == source_hash {
        Some(cached.config)
    } else {
        None
    }
}

/// Write a resolved config to the cache.
fn write_cached_config(cache_dir: &Path, source_hash: u64, config: &ConfigFile) {
    let cached = CachedConfig {
        source_hash,
        config: config.clone(),
    };
    if let Ok(json) = serde_json::to_string(&cached) {
        let _ = std::fs::create_dir_all(cache_dir);
        let _ = std::fs::write(cache_dir.join("config.json"), json);
    }
}

/// Load a JS/TS config file by evaluating it with Node.js.
///
/// Uses a file-content hash cache in `node_modules/.cache/devmoji/` so that
/// Node.js is only spawned when the config file actually changes.
///
/// The config file is expected to have a default export with the config object.
fn load_js_config(path: &Path) -> Option<ConfigFile> {
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };

    // Read source file and compute hash
    let source_bytes = std::fs::read(&abs_path).ok()?;
    let source_hash = hash_bytes(&source_bytes);

    // Check cache
    let cache_dir = find_cache_dir(&abs_path);
    if let Some(ref dir) = cache_dir {
        if let Some(cached) = read_cached_config(dir, source_hash) {
            return Some(cached);
        }
    }

    // Cache miss — evaluate via Node.js
    let path_str = abs_path.to_str()?;
    let ext = abs_path.extension()?.to_str()?;
    let is_ts = matches!(ext, "ts" | "mts");

    let loader_script = format!(
        r#"import('{url}').then(m => {{const c = m.default ?? m; process.stdout.write(JSON.stringify(c));}}).catch(e => {{process.stderr.write(e.message); process.exit(1);}})"#,
        url = path_to_file_url(path_str),
    );

    let json_output = if is_ts {
        try_run_node_with_tsx(&loader_script)
            .or_else(|| try_run_node_strip_types(&loader_script))
    } else {
        try_run_node(&loader_script)
    };

    let output = json_output.or_else(|| {
        eprintln!(
            "devmoji: failed to load config file '{}'. {}",
            path.display(),
            if is_ts {
                "Ensure 'tsx' is installed (npm i -D tsx) or use Node.js >= 22.6 for TypeScript support."
            } else {
                "Ensure Node.js is available on your PATH."
            }
        );
        None
    })?;

    let config: ConfigFile = serde_json::from_str(&output).ok()?;

    // Write to cache for next time
    if let Some(ref dir) = cache_dir {
        write_cached_config(dir, source_hash, &config);
    }

    Some(config)
}

/// Run a script with `node --input-type=module`
fn try_run_node(script: &str) -> Option<String> {
    let output = Command::new("node")
        .args(["--input-type=module", "-e", script])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}

/// Run a script with `tsx` (npx tsx or direct tsx)
fn try_run_node_with_tsx(script: &str) -> Option<String> {
    // Try direct tsx first (globally installed or in node_modules/.bin)
    let output = Command::new("tsx")
        .args(["--eval", script])
        .arg("--input-type=module")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).into_owned());
        }
    }

    // Try via node --import tsx
    let output = Command::new("node")
        .args(["--import", "tsx", "--input-type=module", "-e", script])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}

/// Run a script with Node.js --experimental-strip-types (Node 22.6+)
fn try_run_node_strip_types(script: &str) -> Option<String> {
    let output = Command::new("node")
        .args([
            "--experimental-strip-types",
            "--disable-warning=ExperimentalWarning",
            "--input-type=module",
            "-e",
            script,
        ])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}

/// Convert a file path to a file:// URL for use with dynamic import().
fn path_to_file_url(path: &str) -> String {
    // On Windows, paths need to be converted (C:\foo -> file:///C:/foo)
    if cfg!(windows) {
        let normalized = path.replace('\\', "/");
        if normalized.starts_with('/') {
            format!("file://{}", normalized)
        } else {
            format!("file:///{}", normalized)
        }
    } else {
        format!("file://{}", path)
    }
}
