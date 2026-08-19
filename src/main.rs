use colored::*;
use std::io::{self, Write};

mod ui;

const APP_RUNNING: u32 = 100;
const APP_STOPPED: u32 = 200;

fn help(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'HELP\' command does not require any arguments!"
                .red()
                .bold()
        );
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
        "    {}           {}",
        "CLEAR".cyan().bold(),
        "Clearing the terminal screen".dimmed()
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

fn clear(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'CLEAR\' command does not require any arguments!"
                .red()
                .bold()
        );
        return;
    }

    if let Err(_) = clearscreen::clear() {
        println!("{}", "Failed to clear the terminal screen!".red().bold());
    }
}

fn add(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'ADD\' command requires two arguments!".red().bold()
        );
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    println!("{:.10}", a + b);
}

fn subtract(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'SUBTR\' command requires two arguments!".red().bold()
        );
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    println!("{:.10}", a - b);
}

fn multiply(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'MULTI\' command requires two arguments!".red().bold()
        );
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    println!("{:.10}", a * b);
}

fn divide(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'DIV\' command requires two arguments!".red().bold()
        );
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    println!("{:.10}", a / b);
}

fn exit(cmd: &[&str]) -> u32 {
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'EXIT\' command does not require any arguments!"
                .red()
                .bold()
        );
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
        }
        "CLEAR" => {
            clear(&cmd);
            return APP_RUNNING;
        }
        "ADD" => {
            add(&cmd);
            return APP_RUNNING;
        }
        "SUBTR" => {
            subtract(&cmd);
            return APP_RUNNING;
        }
        "MULTI" => {
            multiply(&cmd);
            return APP_RUNNING;
        }
        "DIV" => {
            divide(&cmd);
            return APP_RUNNING;
        }
        "EXIT" => return exit(&cmd),
        _ => {
            println!("{}", "Invalid command!".red().bold());
            return APP_RUNNING;
        }
    }
}

fn main() {
    let mut app_status = APP_RUNNING;

    ui::app_init();

    while app_status == APP_RUNNING {
        print!("> ");

        if let Err(_) = io::stdout().flush() {
            println!("{}", "Flush failed!".red().bold());
        }

        let mut cmd = String::new();

        if let Err(_) = io::stdin().read_line(&mut cmd) {
            println!("{}", "Failed to read input!".red().bold());
        }

        app_status = exec_cmd(&cmd);
    }
}
