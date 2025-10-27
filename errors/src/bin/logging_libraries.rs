use log::{debug, error, info, warn};

fn main() {
    env_logger::init();

    info!("This is a info message!");
    warn!("This is a warn message!");
    error!("This is an error");
    debug!("This is a debug message!");
}
