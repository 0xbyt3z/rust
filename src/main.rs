use tracing::{info, warn, error};
use rust::logging::logger::init_subscriber;

fn main() {

    init_subscriber();

    info!("This is a info log");
    warn!("This is a warning log");
    error!("This is a error log");
}
