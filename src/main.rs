//! # Weird Calculator
//!
//! A small interactive command-line calculator (REPL).
//!
//! The program prints a banner once at startup, then repeatedly reads a line of
//! input from the user, dispatches it to the command layer, and keeps looping
//! until that layer reports that the application should stop.
//!
//! Module layout:
//! - [`commands`] - command parsing/dispatching plus the command implementations.
//! - [`ui`]       - purely cosmetic terminal output (startup banner).

/// Command parsing, dispatching, and the application status constants.
mod commands;
/// Terminal presentation helpers (startup banner).
mod ui;

use commands::{exec_cmd, APP_RUNNING};
use std::io::{self, Write};
use colored::*;

/// Entry point of the program.
///
/// Runs the read-eval-print loop:
/// 1. Draw the startup banner once.
/// 2. Show the prompt, read one line from standard input.
/// 3. Hand the line over to [`exec_cmd`] and use its return value as the new
///    application status.
///
/// The loop ends as soon as [`exec_cmd`] returns anything other than
/// [`APP_RUNNING`] (in practice `APP_STOPPED`, returned by the `EXIT` command).
fn main() {
    // Application status flag that drives the main loop; starts as "running".
    let mut app_status = APP_RUNNING;

    // Print the ASCII logo and the introductory hint.
    ui::app_init();

    // Keep looping for as long as the command layer reports "still running".
    while app_status == APP_RUNNING {
        print!("> ");

        // `print!` does not append a newline, so stdout must be flushed
        // manually to make sure the prompt shows up before reading input.
        if let Err(_) = io::stdout().flush() {
            println!("{}", "Flush failed!".red().bold());
        }

        // Buffer holding the raw line typed by the user (newline included).
        let mut cmd = String::new();

        // Read one full line from standard input; on failure the buffer stays
        // empty/partial and the error is reported to the user.
        if let Err(_) = io::stdin().read_line(&mut cmd) {
            println!("{}", "Failed to read input!".red().bold());
        }

        // Execute the command and update the status that controls the loop.
        app_status = exec_cmd(&cmd);
    }
}
