use colored::*;

pub mod math;
pub mod utility;

pub const APP_RUNNING: u32 = 100;
pub const APP_STOPPED: u32 = 200;

pub fn exec_cmd(cmd: &str) -> u32 {
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
            math::add(&cmd);
            return APP_RUNNING;
        }
        "SUBTR" => {
            math::subtract(&cmd);
            return APP_RUNNING;
        }
        "MULTI" => {
            math::multiply(&cmd);
            return APP_RUNNING;
        }
        "DIV" => {
            math::divide(&cmd);
            return APP_RUNNING;
        }
        "EXIT" => return utility::exit(&cmd),
        _ => {
            println!("{}", "Invalid command!".red().bold());
            return APP_RUNNING;
        }
    }
}