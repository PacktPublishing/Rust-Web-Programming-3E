use::tracing::Level;
use::tracing_subscriber::FmtSubscriber;

#[cfg(feature = "elastic-logger")]
use super::elastic_actor::send_log;


pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up logger");
}


// Wrapper functions for logging
pub async fn log_info(message: &str) {
    tracing::info!("{}", message);
    #[cfg(feature = "elastic-logger")]
    send_log("INFO", message).await;
}
pub async fn log_warn(message: &str) {
    tracing::warn!("{}", message);
    #[cfg(feature = "elastic-logger")]
    send_log("WARN", message).await;
}
pub async fn log_error(message: &str) {
    tracing::error!("{}", message);
    #[cfg(feature = "elastic-logger")]
    send_log("ERROR", message).await;
}
pub async fn log_debug(message: &str) {
    tracing::debug!("{}", message);
    #[cfg(feature = "elastic-logger")]
    send_log("DEBUG", message).await;
}
pub async fn log_trace(message: &str) {
    tracing::trace!("{}", message);
    #[cfg(feature = "elastic-logger")]
    send_log("TRACE", message).await;
}