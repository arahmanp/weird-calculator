mod app_init;
mod app_status;

fn main() {
    let mut app_status = app_status::APP_RUNNING;

    app_init::app_init();
}
