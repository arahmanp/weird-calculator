//! Command layer of the calculator.
//!
//! This module owns three things:
//! - the application status constants ([`APP_RUNNING`] / [`APP_STOPPED`]) used
//!   by the main loop,
//! - the tokenizer/dispatcher [`exec_cmd`],
//! - the submodules that actually implement the commands ([`math`] and
//!   [`utility`]).
//!
//! Every command is written in upper case and its arguments are separated by
//! whitespace, e.g. `ADD 2 3`.

use colored::*;

/// Arithmetic commands: `ADD`, `SUBTR`, `MULTI`, `DIV`.
pub mod math;
/// Non-arithmetic commands: `HELP`, `CLEAR`, `EXIT`.
pub mod utility;

/// Status meaning "the application should keep running".
///
/// Returned by every command that does not terminate the program.
pub const APP_RUNNING: u32 = 100;

/// Status meaning "the application should shut down".
///
/// Only produced by the `EXIT` command (see [`utility::exit`]).
pub const APP_STOPPED: u32 = 200;

/// Parses a raw input line and runs the matching command.
///
/// The line is trimmed and split on whitespace: the first token is the command
/// name and the remaining tokens are its arguments. The full token slice
/// (command name included) is forwarded to the command implementation, which is
/// why each of them validates `cmd.len()` against the argument count plus one.
///
/// # Arguments
///
/// * `cmd` - the raw line read from standard input, trailing newline included.
///
/// # Returns
///
/// The new application status: [`APP_RUNNING`] in every case except a valid
/// `EXIT` command, which yields [`APP_STOPPED`].
///
/// Empty input and unknown commands are not errors that stop the program: the
/// former is silently ignored, the latter prints an "Invalid command!" message.
pub fn exec_cmd(cmd: &str) -> u32 {
    // Drop the trailing newline and any surrounding spaces.
    let cmd = cmd.trim();

    // Split the line into tokens: cmd[0] is the command, cmd[1..] the arguments.
    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    // A blank line is a no-op; just keep the loop going.
    if cmd.is_empty() {
        return APP_RUNNING;
    }

    // Dispatch on the command name (the first token).
    match cmd[0] {
        // Print the list of available commands.
        "HELP" => {
            utility::help(&cmd);
            return APP_RUNNING;
        }
        // Wipe the terminal screen.
        "CLEAR" => {
            utility::clear(&cmd);
            return APP_RUNNING;
        }
        // Arithmetic: each of these takes exactly two numeric arguments.
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
        // The only command that can change the status: it decides by itself
        // whether the program stops (valid call) or continues (bad arguments).
        "EXIT" => return utility::exit(&cmd),
        // Anything else is not a known command.
        _ => {
            println!("{}", "Invalid command!".red().bold());
            return APP_RUNNING;
        }
    }
}
