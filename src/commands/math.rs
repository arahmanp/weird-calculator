use colored::*;

pub fn add(cmd: &[&str]) {
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

pub fn subtract(cmd: &[&str]) {
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

pub fn multiply(cmd: &[&str]) {
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

pub fn divide(cmd: &[&str]) {
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
