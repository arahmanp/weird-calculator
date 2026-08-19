use colored::*;

pub fn app_init() {
    // Logo dipecah per baris untuk efek gradien
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

    // Array warna untuk membuat gradien Magenta -> Cyan
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

    println!();

    // Print logo dengan efek gradien TrueColor
    for (line, (r, g, b)) in logo_lines.iter().zip(colors.iter()) {
        println!("{}", line.truecolor(*r, *g, *b).bold());
    }

    println!(
        " {} {} {} {}",
        " Weird Calculator "
            .on_custom_color(CustomColor::new(40, 40, 40))
            .bold(),
        "v0.1".yellow().bold(),
        "• by".dimmed(),
        "Andhika Rahman".green().bold()
    );

    println!(
        " Type '{}' to see all available commands.\n",
        "HELP".cyan().bold()
    );
}
