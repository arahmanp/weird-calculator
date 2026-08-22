//! Non-arithmetic commands of the calculator.
//!
//! These commands control the session itself rather than performing a
//! calculation: showing the command reference (`HELP`), clearing the terminal
//! (`CLEAR`), and terminating the program (`EXIT`).
//!
//! Like the [`math`](crate::commands::math) commands, each function receives the
//! full token slice built by [`exec_cmd`](crate::commands::exec_cmd). All three
//! commands take no arguments, so they consider the call valid only when the
//! slice holds exactly one token: the command name.

use colored::*;
use crate::commands::{APP_RUNNING, APP_STOPPED};

/// Handles the `HELP` command: prints the list of available commands.
///
/// # Arguments
///
/// * `cmd` - token slice, expected to be exactly `["HELP"]`.
///
/// Any extra token makes the function print an error and return without showing
/// the command list.
pub fn help(cmd: &[&str]) {
    // `HELP` takes no arguments, so the slice must hold the command name only.
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'HELP\' command does not require any arguments!"
                .red()
                .bold()
        );
        return;
    }

    // Section header.
    println!("{}", " Available commands: ".bold().on_blue());
    println!();

    // Each entry below prints one command, its placeholder arguments (`A`, `B`),
    // and a dimmed description. The spacing inside the format strings is what
    // keeps the description column aligned.
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

    // `EXIT` is highlighted in red because it ends the session.
    println!(
        "    {}            {}",
        "EXIT".red().bold(),
        "Exit the program".dimmed()
    );

    // Trailing blank line so the next prompt is not glued to the list.
    println!("");
}

/// Handles the `CLEAR` command: wipes the terminal screen.
///
/// # Arguments
///
/// * `cmd` - token slice, expected to be exactly `["CLEAR"]`.
///
/// The actual clearing is delegated to the `clearscreen` crate, which picks the
/// right mechanism for the current platform. A failure there is reported but
/// otherwise ignored, so the session continues normally.
pub fn clear(cmd: &[&str]) {
    // `CLEAR` takes no arguments.
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'CLEAR\' command does not require any arguments!"
                .red()
                .bold()
        );
        return;
    }

    // Platform-independent screen clear; report the problem if it fails.
    if let Err(_) = clearscreen::clear() {
        println!("{}", "Failed to clear the terminal screen!".red().bold());
    }
}

/// Handles the `EXIT` command: requests the shutdown of the application.
///
/// This is the only command that can change the application status, so unlike
/// the other command functions it returns a value instead of `()`.
///
/// # Arguments
///
/// * `cmd` - token slice, expected to be exactly `["EXIT"]`.
///
/// # Returns
///
/// [`APP_STOPPED`] when the command is called correctly, which makes the main
/// loop finish; [`APP_RUNNING`] when extra arguments were supplied, leaving the
/// session untouched.
pub fn exit(cmd: &[&str]) -> u32 {
    // `EXIT` takes no arguments; a malformed call must not end the session.
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'EXIT\' command does not require any arguments!"
                .red()
                .bold()
        );
        return APP_RUNNING;
    }

    // Signal the main loop to stop.
    APP_STOPPED
}
