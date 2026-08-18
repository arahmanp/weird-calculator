const APP_RUNNING: u32 = 100;
const APP_STOPPED: u32 = 200;

fn help(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!("The \'HELP\' command does not require any arguments!");
        return;
    }
    
    println!("Available commands:");
    println!("");
    println!("    HELP            Display all available commands");
    println!("    EXIT            Exit the program");
    println!("");
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
}