mod commits;
mod config;
mod devmoji;
mod github_emoji;
mod gitmoji;

use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

use clap::Parser;
use colored::Colorize;

use commits::ConventionalCommits;
use config::Config;
use devmoji::Devmoji;

#[derive(Parser)]
#[command(name = "devmoji", version, about = "Emojify conventional commits")]
struct Cli {
    /// Location of the devmoji.config.json file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// List all known devmojis
    #[arg(short, long)]
    list: bool,

    /// Text to format. Reads from stdin when omitted.
    #[arg(short, long)]
    text: Option<String>,

    /// Lint the conventional commit
    #[arg(long)]
    lint: bool,

    /// Format: unicode, shortcode, devmoji, strip
    #[arg(short, long, default_value = "unicode")]
    format: String,

    /// Process conventional commit headers
    #[arg(long, default_value_t = true)]
    commit: bool,

    /// Do not process conventional commit headers
    #[arg(long)]
    no_commit: bool,

    /// Read last commit message from .git/COMMIT_EDITMSG
    #[arg(short, long)]
    edit: bool,

    /// Format conventional commits similar to git log
    #[arg(long)]
    log: bool,

    /// Use colors for formatting
    #[arg(long)]
    color: Option<bool>,

    /// Don't use colors
    #[arg(long)]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();

    let commit_enabled = cli.commit && !cli.no_commit;
    let use_color = if cli.no_color {
        false
    } else if let Some(c) = cli.color {
        c
    } else {
        atty::is(atty::Stream::Stdout)
    };

    if !use_color {
        colored::control::set_override(false);
    }

    let cfg = Config::load(cli.config.as_deref());
    let dm = Devmoji::new(&cfg);
    let cc = ConventionalCommits::new(&dm, &cfg);

    // --list mode
    if cli.list {
        print_list(&dm, &cfg);
        return;
    }

    // --edit mode
    if cli.edit {
        handle_edit(&dm, &cc, commit_enabled, &cli.format);
        return;
    }

    // --text mode
    if let Some(text) = &cli.text {
        let output = process_text(
            &dm,
            &cc,
            text,
            commit_enabled,
            cli.log,
            &cli.format,
            use_color,
            cli.lint,
        );
        println!("{}", output);
        return;
    }

    // stdin mode
    if !atty::is(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut first_line = true;
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };

            let output = if first_line {
                process_text(
                    &dm,
                    &cc,
                    &line,
                    commit_enabled,
                    cli.log,
                    &cli.format,
                    use_color,
                    cli.lint,
                )
            } else {
                process_text(&dm, &cc, &line, false, cli.log, &cli.format, use_color, false)
            };

            println!("{}", output);
            first_line = false;
        }
        return;
    }

    // No input - show help
    eprintln!("No input provided. Use --text, --edit, or pipe input via stdin.");
    eprintln!("Run with --help for usage information.");
    process::exit(1);
}

fn process_text(
    dm: &Devmoji,
    cc: &ConventionalCommits,
    text: &str,
    commit: bool,
    log: bool,
    format: &str,
    color: bool,
    lint: bool,
) -> String {
    // Lint first if requested
    if lint && commit && !log {
        if let Err(errors) = cc.lint(text) {
            for err in &errors {
                eprintln!("{}", err);
            }
            process::exit(1);
        }
    }

    let result = if log {
        cc.format_log(text, color)
    } else if commit {
        cc.format_commit(text, color)
    } else {
        match format {
            "shortcode" => dm.demojify(text),
            "devmoji" => dm.devmojify(text),
            "strip" => dm.strip(text),
            _ => dm.emojify(text),
        }
    };

    // Apply format conversion if commit/log mode
    if commit || log {
        match format {
            "shortcode" => dm.demojify(&result),
            "devmoji" => dm.devmojify(&result),
            "strip" => dm.strip(&result),
            _ => result,
        }
    } else {
        result
    }
}

fn print_list(dm: &Devmoji, cfg: &Config) {
    for entry in dm.pack() {
        let emoji = dm.get(&entry.emoji);

        let type_prefix = if cfg.types.iter().any(|t| t == &entry.code) {
            format!("{}: ", entry.code)
        } else if entry.code.contains('-') {
            let parts: Vec<&str> = entry.code.splitn(2, '-').collect();
            if cfg.types.iter().any(|t| t == parts[0]) {
                format!("{}({}): ", parts[0], parts[1])
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        println!(
            "{}  {:30} {}{}",
            emoji,
            format!(":{}:", entry.code),
            type_prefix,
            entry.description
        );
    }
}

fn handle_edit(dm: &Devmoji, cc: &ConventionalCommits, commit: bool, format: &str) {
    let git_dir = find_git_dir();
    match git_dir {
        Some(dir) => {
            let msg_file = dir.join("COMMIT_EDITMSG");
            if !msg_file.exists() {
                eprintln!("Could not find {}", msg_file.display());
                process::exit(1);
            }

            let text = match std::fs::read_to_string(&msg_file) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Error reading {}: {}", msg_file.display(), e);
                    process::exit(1);
                }
            };

            // Format without color for file
            let formatted = if commit {
                cc.format_commit(&text, false)
            } else {
                match format {
                    "shortcode" => dm.demojify(&text),
                    "devmoji" => dm.devmojify(&text),
                    "strip" => dm.strip(&text),
                    _ => dm.emojify(&text),
                }
            };

            // Write back
            if let Err(e) = std::fs::write(&msg_file, &formatted) {
                eprintln!("Error writing {}: {}", msg_file.display(), e);
                process::exit(1);
            }

            // Format with color for display
            let display = if commit {
                cc.format_commit(&text, true)
            } else {
                formatted.clone()
            };

            // Print with checkmark
            let first_line = display.lines().next().unwrap_or(&display);
            println!("{} {}", "\u{2714}".green(), first_line);
        }
        None => {
            eprintln!("Could not find .git directory");
            process::exit(1);
        }
    }
}

fn find_git_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let git = dir.join(".git");
        if git.exists() {
            return Some(git);
        }
        if !dir.pop() {
            return None;
        }
    }
}
