use std::io::{self, Write};

const APP_RUNNING: u32 = 100;
const APP_STOPPED: u32 = 200;

fn help(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!("The \'HELP\' command does not require any arguments!");
        return;
    }
    
    println!("Available commands:");
    println!("");
    println!("    HELP            Showing all available commands");
    println!("    ADD A B         Displaying the sum of two numbers A and B");
    println!("    SUBTR A B       Displaying the subtraction of two numbers A and B");
    println!("    EXIT            Exit the program");
    println!("");
}

fn add(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'ADD\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a + b);
}

fn subtract(cmd: &[&str]) {
    if cmd.len() != 3 {
        println!("The \'SUBTR\' command requires two arguments!");
        return;
    }

    let a = match cmd[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    let b = match cmd[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The argument has to be a number!");
            return;
        }
    };

    println!("{}", a - b);
}

fn exit(cmd: &[&str]) -> u32 {
    if cmd.len() != 1 {
        println!("The \'EXIT\' command does not require any arguments!");
        return APP_RUNNING;
    }

    APP_STOPPED
}

fn exec_cmd(cmd: &str) -> u32 {
    let cmd = cmd.trim();

    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if cmd.is_empty() {
        return APP_RUNNING;
    }

    match cmd[0] {
        "HELP" => {
            help(&cmd);
            return APP_RUNNING;
        },
        "ADD" => {
            add(&cmd);
            return APP_RUNNING;
        },
        "SUBTR" => {
            subtract(&cmd);
            return APP_RUNNING;
        },
        "EXIT" => return exit(&cmd),
        _ => {
            println!("Invalid command!");
            return APP_RUNNING;
        }
    }
}

fn app_init() {
    let logo = r#"
     _       ________________  ____ 
    | |     / / ____/  _/ __ \/ __ \
    | | /| / / __/  / // /_/ / / / /
    | |/ |/ / /____/ // _, _/ /_/ / 
    |__/|__/_____/___/_/ |_/_____/  
                                    
       _________    __    ________  ____    ___  __________  ____ 
      / ____/   |  / /   / ____/ / / / /   /   |/_  __/ __ \/ __ \
     / /   / /| | / /   / /   / /   / /   / /| | / / / / / / /_/ /
    / /___/ ___ |/ /___/ /___/ /_/ / /___/ ___ |/ / / /_/ / _, _/ 
    \____/_/  |_/_____/\____/\____/_____/_/  |_/_/  \____/_/ |_|  
    "#;

    println!("{}", logo);
    
    println!("Weird Calculator v0.1 - by Andhika Rahman");
    println!("Type \'HELP\' to see all available commands.");
}

fn main() {
    let mut app_status = APP_RUNNING;
    
    app_init();

    while app_status == APP_RUNNING {
        print!("> ");

        io::stdout()
            .flush()
            .unwrap();

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read input!");

        app_status = exec_cmd(&cmd);
    }
}