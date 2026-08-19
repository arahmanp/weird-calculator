mod commands;
mod ui;

use commands::{exec_cmd, APP_RUNNING};
use std::io::{self, Write};
use colored::*;

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
