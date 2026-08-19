use colored::*;
use crate::status;

pub fn help(cmd: &[&str]) {
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
        "EXIT".red().bold(),
        "Exit the program".dimmed()
    );

    println!("");
}

pub fn clear(cmd: &[&str]) {
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

pub fn exit(cmd: &[&str]) -> u32 {
    if cmd.len() != 1 {
        println!(
            "{}",
            "The \'EXIT\' command does not require any arguments!"
                .red()
                .bold()
        );
        return status::APP_RUNNING;
    }

    status::APP_STOPPED
}
