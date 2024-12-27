use log::{debug, error, info, trace, warn};
use solana_logger::*;

fn main() {
    // Set up the logger with a default filter level of "info"
    setup_with_default("info");

    // Log messages at different levels
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
}
