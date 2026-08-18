use crate::app_status;

pub fn exec_cmd(cmd: &str) -> u32 {
    let cmd = cmd.trim();

    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if cmd.is_empty() {
        return app_status::APP_RUNNING;
    }

    match cmd[0] {
        "help" => {
            help(&cmd);
            return app_status::APP_RUNNING;
        },
        "exit" => return exit(&cmd),
        _ => {
            println!("Invalid command!");
            return app_status::APP_RUNNING;
        }
    }
}