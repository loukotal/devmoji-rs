use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::Config;
use crate::devmoji::Devmoji;

static COMMIT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?mi)(?P<type>:?[a-z][a-z0-9-]*)(?:\((?P<scope>[a-z0-9-]+)\))?(?P<breaking>!?):\s*(?:(?P<other>(?::[a-z0-9_+-]+:\s*)+)\s*)?")
        .unwrap()
});

static BREAKING_CHANGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*BREAKING CHANGE").unwrap());

static SHORTCODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r":([a-zA-Z0-9_\-+]+):").unwrap());

pub struct ConventionalCommits<'a> {
    devmoji: &'a Devmoji,
    config: &'a Config,
}

impl<'a> ConventionalCommits<'a> {
    pub fn new(devmoji: &'a Devmoji, config: &'a Config) -> Self {
        ConventionalCommits { devmoji, config }
    }

    pub fn format_commit(&self, text: &str, color: bool) -> String {
        self.format(text, true, color)
    }

    pub fn format_log(&self, text: &str, color: bool) -> String {
        self.format(text, false, color)
    }

    fn format(&self, text: &str, first_only: bool, color: bool) -> String {
        // First devmojify to normalize existing emoji to devmoji shortcodes
        let text = self.devmoji.devmojify(text);

        let has_breaking = BREAKING_CHANGE_RE.is_match(&text);

        let mut result = String::new();
        let mut last_end = 0;
        let mut found_first = false;

        for caps in COMMIT_RE.captures_iter(&text) {
            let m = caps.get(0).unwrap();

            if first_only && m.start() != 0 {
                continue;
            }
            if first_only && found_first {
                continue;
            }

            let commit_type = caps.name("type").unwrap().as_str();

            // Skip if type starts with ':' (already a shortcode)
            if commit_type.starts_with(':') {
                continue;
            }

            let scope = caps.name("scope").map(|m| m.as_str());
            let breaking = caps.name("breaking").map(|m| m.as_str()) == Some("!");
            let other = caps.name("other").map(|m| m.as_str()).unwrap_or("");

            let emojis =
                self.format_emoji(commit_type, scope, other, breaking || has_breaking);

            // Build replacement
            let mut replacement = String::new();
            if color {
                replacement.push_str(&commit_type.blue().to_string());
                if let Some(s) = scope {
                    replacement.push('(');
                    replacement.push_str(&s.bold().to_string());
                    replacement.push(')');
                }
            } else {
                replacement.push_str(commit_type);
                if let Some(s) = scope {
                    replacement.push('(');
                    replacement.push_str(s);
                    replacement.push(')');
                }
            }
            if breaking || has_breaking {
                replacement.push('!');
            }
            replacement.push_str(": ");
            replacement.push_str(&emojis);
            if !emojis.is_empty() {
                replacement.push(' ');
            }

            result.push_str(&text[last_end..m.start()]);
            result.push_str(&replacement);
            last_end = m.end();
            found_first = true;
        }

        result.push_str(&text[last_end..]);

        // Now convert remaining shortcodes based on format
        self.devmoji.emojify(&result)
    }

    fn format_emoji(
        &self,
        commit_type: &str,
        scope: Option<&str>,
        other: &str,
        breaking: bool,
    ) -> String {
        let mut emojis: Vec<String> = Vec::new();

        // Breaking change emoji
        if breaking {
            emojis.push(self.devmoji.get("boom"));
        }

        // Type emoji
        let type_emoji = self.lookup_pack_code(commit_type);

        // Scope handling
        if let Some(scope) = scope {
            let compound = format!("{}-{}", commit_type, scope);
            if let Some(e) = self.lookup_pack_code(&compound) {
                // Use compound emoji instead of type emoji
                push_unique(&mut emojis, e);
            } else {
                // Use type emoji + scope emoji
                if let Some(e) = type_emoji {
                    push_unique(&mut emojis, e);
                }
                if let Some(e) = self.lookup_pack_code(scope) {
                    push_unique(&mut emojis, e);
                }
            }
        } else if let Some(e) = type_emoji {
            push_unique(&mut emojis, e);
        }

        // Parse other shortcodes
        for caps in SHORTCODE_RE.captures_iter(other) {
            let code = &caps[1];
            let emoji = self.devmoji.get(code);
            push_unique(&mut emojis, emoji);
        }

        emojis.join(" ")
    }

    fn lookup_pack_code(&self, code: &str) -> Option<String> {
        for entry in self.devmoji.pack() {
            if entry.code == code {
                return Some(self.devmoji.get(&entry.emoji));
            }
        }
        None
    }

    pub fn lint(&self, text: &str) -> Result<(), Vec<String>> {
        let first_line = text.lines().next().unwrap_or("");

        // Skip linting for special commits
        if first_line.starts_with("Merge branch")
            || first_line.starts_with("fixup!")
            || first_line.starts_with("squash!")
            || first_line.starts_with("Revert")
            || first_line.starts_with("revert")
        {
            return Ok(());
        }

        let mut errors = Vec::new();

        if let Some(caps) = COMMIT_RE.captures(first_line) {
            if caps.get(0).unwrap().start() != 0 {
                errors.push(format!(
                    "Expecting a commit message like: type(scope): description"
                ));
                return Err(errors);
            }

            let commit_type = caps.name("type").unwrap().as_str();
            if !self.config.types.iter().any(|t| t == commit_type) {
                errors.push(format!(
                    "Type should be one of: {}",
                    self.config.types.join(", ")
                ));
            }

            // Check if there's a description after the match
            let m = caps.get(0).unwrap();
            let rest = &first_line[m.end()..].trim();
            if rest.is_empty() {
                errors.push("Missing description".to_string());
            }
        } else {
            errors.push("Expecting a commit message like: type(scope): description".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn push_unique(vec: &mut Vec<String>, item: String) {
    if !item.is_empty() && !vec.contains(&item) {
        vec.push(item);
    }
}
