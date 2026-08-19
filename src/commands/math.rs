//! Arithmetic commands of the calculator.
//!
//! Every function in this module implements one binary operation and shares the
//! same shape:
//!
//! 1. it receives the whole token slice produced by
//!    [`exec_cmd`](crate::commands::exec_cmd), so `cmd[0]` is the command name
//!    itself and `cmd[1]` / `cmd[2]` are the operands;
//! 2. it rejects the call unless there are exactly two operands
//!    (`cmd.len() != 3`);
//! 3. it parses both operands as [`f64`], aborting with a message if either one
//!    is not a valid number;
//! 4. it prints the result with ten digits after the decimal point.
//!
//! None of these functions return a value or propagate errors: every problem is
//! reported directly to the terminal and simply cancels the operation, leaving
//! the REPL running.

use colored::*;

/// Handles the `ADD A B` command: prints the sum `A + B`.
///
/// # Arguments
///
/// * `cmd` - token slice `["ADD", "A", "B"]`.
///
/// Prints an error and returns early when the argument count is wrong or when
/// an operand cannot be parsed as an [`f64`].
pub fn add(cmd: &[&str]) {
    // Expect exactly the command name plus two operands.
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'ADD\' command requires two arguments!".red().bold()
        );
        return;
    }

    // First operand.
    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Second operand.
    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Fixed precision output: always ten decimal places.
    println!("{:.10}", a + b);
}

/// Handles the `SUBTR A B` command: prints the difference `A - B`.
///
/// # Arguments
///
/// * `cmd` - token slice `["SUBTR", "A", "B"]`.
///
/// Prints an error and returns early when the argument count is wrong or when
/// an operand cannot be parsed as an [`f64`].
pub fn subtract(cmd: &[&str]) {
    // Expect exactly the command name plus two operands.
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'SUBTR\' command requires two arguments!".red().bold()
        );
        return;
    }

    // Minuend.
    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Subtrahend.
    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Fixed precision output: always ten decimal places.
    println!("{:.10}", a - b);
}

/// Handles the `MULTI A B` command: prints the product `A * B`.
///
/// # Arguments
///
/// * `cmd` - token slice `["MULTI", "A", "B"]`.
///
/// Prints an error and returns early when the argument count is wrong or when
/// an operand cannot be parsed as an [`f64`].
pub fn multiply(cmd: &[&str]) {
    // Expect exactly the command name plus two operands.
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'MULTI\' command requires two arguments!".red().bold()
        );
        return;
    }

    // First factor.
    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Second factor.
    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Fixed precision output: always ten decimal places.
    println!("{:.10}", a * b);
}

/// Handles the `DIV A B` command: prints the quotient `A / B`.
///
/// # Arguments
///
/// * `cmd` - token slice `["DIV", "A", "B"]`.
///
/// Prints an error and returns early when the argument count is wrong or when
/// an operand cannot be parsed as an [`f64`].
///
/// Division by zero is not treated as an error here: it follows IEEE 754
/// floating point rules and prints `inf`, `-inf`, or `NaN`.
pub fn divide(cmd: &[&str]) {
    // Expect exactly the command name plus two operands.
    if cmd.len() != 3 {
        println!(
            "{}",
            "The \'DIV\' command requires two arguments!".red().bold()
        );
        return;
    }

    // Dividend.
    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Divisor.
    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "The argument has to be a number!".red().bold());
            return;
        }
    };

    // Fixed precision output: always ten decimal places.
    println!("{:.10}", a / b);
}
