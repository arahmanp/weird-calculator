//! Terminal presentation layer.
//!
//! This module contains only cosmetic output: the startup banner drawn once
//! before the REPL begins. It holds no application state and never reads user
//! input.

use colored::*;

/// Prints the startup banner.
///
/// Draws the ASCII-art "WEIRD CALCULATOR" logo with a magenta-to-cyan gradient,
/// followed by the version/author line and a hint pointing at the `HELP`
/// command.
///
/// The gradient is produced by zipping the logo lines with a palette of RGB
/// triplets, so each line is rendered in its own truecolor shade. Because
/// [`Iterator::zip`] stops at the shorter of the two iterators, the palette is
/// intentionally at least as long as the logo.
///
/// Truecolor output requires a terminal with 24-bit color support; elsewhere the
/// `colored` crate degrades the styling on its own.
pub fn app_init() {
    // ASCII-art logo, one entry per line. Raw strings keep the backslashes
    // literal, which the artwork relies on.
    let logo_lines = [
        r#"     _       ________________  ____ "#,
        r#"    | |     / / ____/  _/ __ \/ __ \"#,
        r#"    | | /| / / __/  / // /_/ / / / /"#,
        r#"    | |/ |/ / /____/ // _, _/ /_/ / "#,
        r#"    |__/|__/_____/___/_/ |_/_____/  "#,
        r#"                                    "#,
        r#"        _________    __    ________  ____    ___  __________  ____ "#,
        r#"       / ____/   |  / /   / ____/ / / / /   /   |/_  __/ __ \/ __ \"#,
        r#"      / /   / /| | / /   / /   / / / / /   / /| | / / / / / / /_/ /"#,
        r#"     / /___/ ___ |/ /___/ /___/ /_/ / /___/ ___ |/ / / /_/ / _, _/ "#,
        r#"     \____/_/  |_/_____/\____/\____/_____/_/  |_/_/  \____/_/ |_|  "#,
    ];

    // Gradient palette as (red, green, blue) triplets, going from magenta at the
    // top to cyan at the bottom. One entry per logo line.
    let colors = [
        (255, 0, 255),
        (220, 30, 255),
        (180, 60, 255),
        (140, 90, 255),
        (100, 120, 255),
        (80, 150, 255),
        (60, 180, 255),
        (40, 210, 255),
        (20, 230, 255),
        (0, 255, 255),
        (0, 255, 255),
    ];

    // Blank line separating the banner from whatever was on screen before.
    println!();

    // Pair each logo line with its color and print it in that shade.
    for (line, (r, g, b)) in logo_lines.iter().zip(colors.iter()) {
        println!("{}", line.truecolor(*r, *g, *b).bold());
    }

    // Separate the banner and the title bar
    println!();

    // Title bar: application name on a dark background, then version and author.
    println!(
        " {} {} {} {}",
        " Weird Calculator "
            .on_custom_color(CustomColor::new(40, 40, 40))
            .bold(),
        "v0.1".yellow().bold(),
        "• by".dimmed(),
        "Andhika Rahman".green().bold()
    );

    // Hint for first-time users, followed by a blank line before the prompt.
    println!(
        " Type '{}' to see all available commands.\n",
        "HELP".cyan().bold()
    );
}
