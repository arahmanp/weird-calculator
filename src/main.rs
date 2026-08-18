const APP_RUNNING: u32 = 100;
const APP_STOPPED: u32 = 200;

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