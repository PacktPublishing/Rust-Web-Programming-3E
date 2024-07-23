use tracing::Level;
use tracing_subscriber::FmtSubscriber;


pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up logger");
}


// Wrapper functions for logging
pub fn log_info(message: &str) {
    tracing::info!("{}", message);
}

pub fn log_warn(message: &str) {
    tracing::warn!("{}", message);
}

pub fn log_error(message: &str) {
    tracing::error!("{}", message);
}

pub fn log_debug(message: &str) {
    tracing::debug!("{}", message);
}

pub fn log_trace(message: &str) {
    tracing::trace!("{}", message);
}