use std::io::{self, Write};
use colored::*;

const APP_RUNNING: u32 = 100;
const APP_STOPPED: u32 = 200;

fn help(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!("{}", "The \'HELP\' command does not require any arguments!".red().bold());
        return;
    }
    
    println!("{}", " Available commands: ".bold().on_blue());
    println!();

    // Command + Argumen + Deskripsi
    println!(
        "    {}            {}",
        "HELP".cyan().bold(),
        "Showing all available commands".dimmed()
    );

    println!(
        "    {} {} {}         {}",
        "ADD".cyan().bold(),
        "A".yellow(),
        "B".yellow(),
        "Displaying the sum of two numbers A and B".dimmed()
    );

    println!(
        "    {} {} {}       {}",
        "SUBTR".cyan().bold(),
        "A".yellow(),
        "B".yellow(),
        "Displaying the subtraction of two numbers A and B".dimmed()
    );

    println!(
        "    {} {} {}       {}",
        "MULTI".cyan().bold(),
        "A".yellow(),
        "B".yellow(),
        "Displaying the multiplication of two numbers A and B".dimmed()
    );

    println!(
        "    {} {} {}         {}",
        "DIV".cyan().bold(),
        "A".yellow(),
        "B".yellow(),
        "Displaying the division of two numbers A and B".dimmed()
    );

    println!(
        "    {}            {}",
        "EXIT".red().bold(), // EXIT diberi warna merah indikasi keluar
        "Exit the program".dimmed()
    );

    println!("");
}

fn add(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'ADD\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a + b);
}

fn subtract(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'SUBTR\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a - b);
}

fn multiply(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'MULTI\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a * b);
}

fn divide(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'DIV\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a / b);
}

fn exit(cmd: &[&str]) -> u32 {
    if cmd.len() != 1 {
        println!("The \'EXIT\' command does not require any arguments!");
        return APP_RUNNING;
    }

    APP_STOPPED
}

fn exec_cmd(cmd: &str) -> u32 {
    let cmd = cmd.trim();

    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if cmd.is_empty() {
        return APP_RUNNING;
    }

    match cmd[0] {
        "HELP" => {
            help(&cmd);
            return APP_RUNNING;
        },
        "ADD" => {
            add(&cmd);
            return APP_RUNNING;
        },
        "SUBTR" => {
            subtract(&cmd);
            return APP_RUNNING;
        },
        "MULTI" => {
            multiply(&cmd);
            return APP_RUNNING;
        },
        "DIV" => {
            divide(&cmd);
            return APP_RUNNING;
        },
        "EXIT" => return exit(&cmd),
        _ => {
            println!("Invalid command!");
            return APP_RUNNING;
        }
    }
}

fn app_init() {
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
        r#"      / /   / /| | / /   / /   / /   / /   / /| | / / / / / / /_/ /"#,
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
        " Weird Calculator ".on_custom_color(CustomColor::new(40, 40, 40)).bold(),
        "v0.1".yellow().bold(),
        "• by".dimmed(),
        "Andhika Rahman".green().bold()
    );

    println!(
        " Type '{}' to see all available commands.\n",
        "HELP".cyan().bold()
    );
}

fn main() {
    let mut app_status = APP_RUNNING;
    
    app_init();

    while app_status == APP_RUNNING {
        print!("> ");

        io::stdout()
            .flush()
            .unwrap();

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read input!");

        app_status = exec_cmd(&cmd);
    }
}