use colored::*;
use std::io::{self, Write};

mod ui;
mod utility;

pub const APP_RUNNING: u32 = 100;
pub const APP_STOPPED: u32 = 200;

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

fn exec_cmd(cmd: &str) -> u32 {
    let cmd = cmd.trim();

    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if cmd.is_empty() {
        return APP_RUNNING;
    }

    match cmd[0] {
        "HELP" => {
            utility::help(&cmd);
            return APP_RUNNING;
        }
        "CLEAR" => {
            utility::clear(&cmd);
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
        "EXIT" => return utility::exit(&cmd),
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
