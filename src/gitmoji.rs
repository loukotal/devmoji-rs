use std::collections::HashMap;

use once_cell::sync::Lazy;

pub struct GitmojiEntry {
    pub code: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
}

pub static GITMOJIS: Lazy<Vec<GitmojiEntry>> = Lazy::new(|| {
    vec![
        GitmojiEntry { code: "art", emoji: "\u{1f3a8}", description: "Improving structure / format of the code." },
        GitmojiEntry { code: "zap", emoji: "\u{26a1}\u{fe0f}", description: "Improving performance." },
        GitmojiEntry { code: "fire", emoji: "\u{1f525}", description: "Removing code or files." },
        GitmojiEntry { code: "bug", emoji: "\u{1f41b}", description: "Fixing a bug." },
        GitmojiEntry { code: "ambulance", emoji: "\u{1f691}", description: "Critical hotfix." },
        GitmojiEntry { code: "sparkles", emoji: "\u{2728}", description: "Introducing new features." },
        GitmojiEntry { code: "pencil", emoji: "\u{1f4dd}", description: "Writing docs." },
        GitmojiEntry { code: "rocket", emoji: "\u{1f680}", description: "Deploying stuff." },
        GitmojiEntry { code: "lipstick", emoji: "\u{1f484}", description: "Updating the UI and style files." },
        GitmojiEntry { code: "tada", emoji: "\u{1f389}", description: "Initial commit." },
        GitmojiEntry { code: "white_check_mark", emoji: "\u{2705}", description: "Updating tests." },
        GitmojiEntry { code: "lock", emoji: "\u{1f512}\u{fe0f}", description: "Fixing security issues." },
        GitmojiEntry { code: "apple", emoji: "\u{1f34e}", description: "Fixing something on macOS." },
        GitmojiEntry { code: "penguin", emoji: "\u{1f427}", description: "Fixing something on Linux." },
        GitmojiEntry { code: "checkered_flag", emoji: "\u{1f3c1}", description: "Fixing something on Windows." },
        GitmojiEntry { code: "robot", emoji: "\u{1f916}", description: "Fixing something on Android." },
        GitmojiEntry { code: "green_apple", emoji: "\u{1f34f}", description: "Fixing something on iOS." },
        GitmojiEntry { code: "bookmark", emoji: "\u{1f516}", description: "Releasing / Version tags." },
        GitmojiEntry { code: "rotating_light", emoji: "\u{1f6a8}", description: "Removing linter warnings." },
        GitmojiEntry { code: "construction", emoji: "\u{1f6a7}", description: "Work in progress." },
        GitmojiEntry { code: "green_heart", emoji: "\u{1f49a}", description: "Fixing CI Build." },
        GitmojiEntry { code: "arrow_down", emoji: "\u{2b07}\u{fe0f}", description: "Downgrading dependencies." },
        GitmojiEntry { code: "arrow_up", emoji: "\u{2b06}\u{fe0f}", description: "Upgrading dependencies." },
        GitmojiEntry { code: "pushpin", emoji: "\u{1f4cc}", description: "Pinning dependencies to specific versions." },
        GitmojiEntry { code: "construction_worker", emoji: "\u{1f477}", description: "Adding CI build system." },
        GitmojiEntry { code: "chart_with_upwards_trend", emoji: "\u{1f4c8}", description: "Adding analytics or tracking code." },
        GitmojiEntry { code: "recycle", emoji: "\u{267b}\u{fe0f}", description: "Refactoring code." },
        GitmojiEntry { code: "whale", emoji: "\u{1f433}", description: "Work about Docker." },
        GitmojiEntry { code: "heavy_plus_sign", emoji: "\u{2795}", description: "Adding a dependency." },
        GitmojiEntry { code: "heavy_minus_sign", emoji: "\u{2796}", description: "Removing a dependency." },
        GitmojiEntry { code: "wrench", emoji: "\u{1f527}", description: "Changing configuration files." },
        GitmojiEntry { code: "globe_with_meridians", emoji: "\u{1f310}", description: "Internationalization and localization." },
        GitmojiEntry { code: "pencil2", emoji: "\u{270f}\u{fe0f}", description: "Fixing typos." },
        GitmojiEntry { code: "poop", emoji: "\u{1f4a9}", description: "Writing bad code that needs to be improved." },
        GitmojiEntry { code: "rewind", emoji: "\u{23ea}", description: "Reverting changes." },
        GitmojiEntry { code: "twisted_rightwards_arrows", emoji: "\u{1f500}", description: "Merging branches." },
        GitmojiEntry { code: "package", emoji: "\u{1f4e6}\u{fe0f}", description: "Updating compiled files or packages." },
        GitmojiEntry { code: "alien", emoji: "\u{1f47d}", description: "Updating code due to external API changes." },
        GitmojiEntry { code: "truck", emoji: "\u{1f69a}", description: "Moving or renaming files." },
        GitmojiEntry { code: "page_facing_up", emoji: "\u{1f4c4}", description: "Adding or updating license." },
        GitmojiEntry { code: "boom", emoji: "\u{1f4a5}", description: "Introducing breaking changes." },
        GitmojiEntry { code: "bento", emoji: "\u{1f371}", description: "Adding or updating assets." },
        GitmojiEntry { code: "ok_hand", emoji: "\u{1f44c}", description: "Updating code due to code review changes." },
        GitmojiEntry { code: "wheelchair", emoji: "\u{267f}\u{fe0f}", description: "Improving accessibility." },
        GitmojiEntry { code: "bulb", emoji: "\u{1f4a1}", description: "Documenting source code." },
        GitmojiEntry { code: "beers", emoji: "\u{1f37b}", description: "Writing code drunkenly." },
        GitmojiEntry { code: "speech_balloon", emoji: "\u{1f4ac}", description: "Updating text and literals." },
        GitmojiEntry { code: "card_file_box", emoji: "\u{1f5c3}", description: "Performing database related changes." },
        GitmojiEntry { code: "loud_sound", emoji: "\u{1f50a}", description: "Adding logs." },
        GitmojiEntry { code: "mute", emoji: "\u{1f507}", description: "Removing logs." },
        GitmojiEntry { code: "busts_in_silhouette", emoji: "\u{1f465}", description: "Adding contributor(s)." },
        GitmojiEntry { code: "children_crossing", emoji: "\u{1f6b8}", description: "Improving user experience / usability." },
        GitmojiEntry { code: "building_construction", emoji: "\u{1f3d7}", description: "Making architectural changes." },
        GitmojiEntry { code: "iphone", emoji: "\u{1f4f1}", description: "Working on responsive design." },
        GitmojiEntry { code: "clown_face", emoji: "\u{1f921}", description: "Mocking things." },
        GitmojiEntry { code: "egg", emoji: "\u{1f95a}", description: "Adding an easter egg." },
        GitmojiEntry { code: "see_no_evil", emoji: "\u{1f648}", description: "Adding or updating a .gitignore file." },
        GitmojiEntry { code: "camera_flash", emoji: "\u{1f4f8}", description: "Adding or updating snapshots." },
        GitmojiEntry { code: "alembic", emoji: "\u{2697}", description: "Experimenting new things." },
        GitmojiEntry { code: "mag", emoji: "\u{1f50d}", description: "Improving SEO." },
        GitmojiEntry { code: "wheel_of_dharma", emoji: "\u{2638}\u{fe0f}", description: "Work about Kubernetes." },
        GitmojiEntry { code: "label", emoji: "\u{1f3f7}\u{fe0f}", description: "Adding or updating types (Flow, TypeScript)." },
        GitmojiEntry { code: "seedling", emoji: "\u{1f331}", description: "Adding or updating seed files." },
        GitmojiEntry { code: "triangular_flag_on_post", emoji: "\u{1f6a9}", description: "Adding, updating, or removing feature flags." },
        GitmojiEntry { code: "goal_net", emoji: "\u{1f945}", description: "Catching errors." },
        GitmojiEntry { code: "dizzy", emoji: "\u{1f4ab}", description: "Adding or updating animations and transitions." },
        GitmojiEntry { code: "wastebasket", emoji: "\u{1f5d1}", description: "Deprecating code that needs to be cleaned up." },
    ]
});

pub static GITMOJI_MAP: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(GITMOJIS.len());
    for (i, entry) in GITMOJIS.iter().enumerate() {
        m.insert(entry.code, i);
    }
    m
});
