use colored::*;
use std::io::{self, Write};

mod math;
mod ui;
mod utility;
mod status;

fn exec_cmd(cmd: &str) -> u32 {
    let cmd = cmd.trim();

    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if cmd.is_empty() {
        return status::APP_RUNNING;
    }

    match cmd[0] {
        "HELP" => {
            utility::help(&cmd);
            return status::APP_RUNNING;
        }
        "CLEAR" => {
            utility::clear(&cmd);
            return status::APP_RUNNING;
        }
        "ADD" => {
            math::add(&cmd);
            return status::APP_RUNNING;
        }
        "SUBTR" => {
            math::subtract(&cmd);
            return status::APP_RUNNING;
        }
        "MULTI" => {
            math::multiply(&cmd);
            return status::APP_RUNNING;
        }
        "DIV" => {
            math::divide(&cmd);
            return status::APP_RUNNING;
        }
        "EXIT" => return utility::exit(&cmd),
        _ => {
            println!("{}", "Invalid command!".red().bold());
            return status::APP_RUNNING;
        }
    }
}

fn main() {
    let mut app_status = status::APP_RUNNING;

    ui::app_init();

    while app_status == status::APP_RUNNING {
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
